mod util;
use util::*;

pub fn squash(z: Vec2) -> Vec2 {
    vec2(1.0 - (-z.x).exp(), z.y)
}

// wres wtrans? maybe yea

pub struct Reverb {
    mem: Vec2,
    wr: Vec2,
    wt: Vec2,
}
impl Reverb {
    pub fn new(w: Vec2) -> Self {
        Self {
            mem: vec2(0.0, 0.0),
            wr: w,
            wt: w,
        }
    }
    pub fn new2(wr: Vec2, wt: Vec2) -> Self {
        Self {
            mem: vec2(0.0, 0.0),
            wr,
            wt,
        }
    }
    pub fn tick(&mut self, z: Vec2) -> Vec2 {
        let prev = self.mem.cmul_pol(self.wr);
        let mut z = z.cadd_pol(prev);
        z.x /= 2.0;
        // let z = squash(z);
        self.mem = z;
        return z;
    }
}
pub struct ReverbChain {
    reverbs: Vec<Reverb>,
    wn: Vec2,
}
impl ReverbChain {
    pub fn new(w: Vec2, n: usize, wn: Vec2) -> Self {
        Self {
            reverbs: std::iter::repeat(w).map(|w| Reverb::new(w)).take(n).collect(),
            wn: wn,
        }
    }
    pub fn tick(&mut self, mut z: Vec2) -> Vec2 {
        // look this tick needs to be energy conserving like the wt and wr
        // is it like a series of transmission lines that slowly losing energy
        // i guess so yeah
        // the idea is you kind of dont want reflections in your transmission lines hey, and you waveguides.
        let mut z = self.reverbs.last().unwrap().mem.cmul_pol(self.wn).cadd_pol(z);
        for reverb in self.reverbs.iter_mut() {
            z = reverb.tick(z);
        }
        z
    }
    pub fn tick_nonlinear(&mut self, mut z: Vec2) -> Vec2 {
        let mut z = self.reverbs.last().unwrap().mem.cmul_pol(self.wn).cadd_pol(z);
        for reverb in self.reverbs.iter_mut() {
            z = reverb.tick(z);
            z = vec2(1.0 - (-z.x).exp(), z.y);
        }
        z = vec2(1.0 - (-z.x).exp(), z.y);
        z
    }
    pub fn tick_parallel(&mut self, z: Vec2) -> Vec2 {
        let mut acc = vec2(0.0, 0.0);
        for reverb in self.reverbs.iter_mut() {
            let z = reverb.tick(z);
            // let z = squash(z);
            acc += z;
        }
        acc
    }
}
pub fn pulses(t: usize, n: usize) -> impl Iterator<Item = Vec2> {
    (0..n).map(move |i| {
        let m = i % t == 0;
        let mf = if m { 1.0 } else { 0.0 };
        vec2(mf, i as f32)
    })
}
pub fn duty_square(n_up: usize, n_down: usize, n: usize) -> impl Iterator<Item = Vec2> {
    (0..n).map(move |i| {
        let i = i % (n_up+n_down);
        let mf = if i < n_up { 1.0 } else { 0.0 };
        vec2(mf, i as f32)
    })
}
pub fn tone(fs: usize, n: usize, f: f32) -> impl Iterator<Item = Vec2> {
    (0..n).map(move |i| {
        let phase = 2.0 * PI * f / fs as f32 * i as f32;
        vec2(1.0, phase)
    })
}
pub fn sweep(fs: usize, n: usize, fstart: f32, fend: f32) -> impl Iterator<Item = Vec2> {
    let mut phase = 0.0;
    (0..n).map(move |i| {
        let alpha = i as f32 / (n - 1) as f32;
        let log_fstart = fstart.log10();
        let log_fend = fend.log10();
        let log_freq = log_fstart * (1.0 - alpha) + log_fend * alpha;   // haha lerp and loglerp
        let freq = 10.0_f32.powf(log_freq);
        phase += 2.0 * PI * freq / fs as f32;

        vec2(1.0, phase)
    })
}

// wn sets boundary u know
pub fn series_exp(file_name: &str, w: Vec2, wn: Vec2, n: usize) {
    let mut rv1 = ReverbChain::new(w, n, wn); // oh yeah whats the phase meant to be daddy
    let mut out1 = vec![];
    let s = pulses(32000, 44100 * 3)
        .map(|z| vec2(z.x*10000.0, z.y));
    for z in s {
        let z = squash(rv1.tick(z));
        // let z = rv1.tick_nonlinear(z);
        out1.push(z);
    }
    out1.iter_mut().for_each(|z| *z = z.pol2cart());
    out1.save(44100, file_name);
}


fn main() {
    series_exp("outwg3.wav", 
        vec2(0.999, 0.01),
        vec2(1.0, 0.0),
        50,
    );
}
