mod util;
use util::*;

pub fn squash(z: Vec2) -> Vec2 {
    vec2(1.0 - (-z.x*10.0).exp(), z.y)
}

pub struct Reverb {
    mem: Vec2,
    w: Vec2,
}
impl Reverb {
    pub fn new(w: Vec2) -> Self {
        Self {
            mem: vec2(0.0, 0.0),
            w,
        }
    }
    pub fn tick(&mut self, z: Vec2) -> Vec2 {
        let prev = self.mem.cmul_pol(self.w);
        let zcart = prev.pol2cart() + z.pol2cart();
        let zpol = zcart.cart2pol();
        self.mem = zpol;
        // assert!(zpol.x < 2000.0);
        return zpol;
    }
}
pub struct ReverbChain {
    reverbs: Vec<Reverb>,
}
impl ReverbChain {
    pub fn new(w: Vec<Vec2>) -> Self {
        Self {
            reverbs: w.into_iter().map(|w| Reverb::new(w)).collect(),
        }
    }
    pub fn tick(&mut self, mut z: Vec2) -> Vec2 {
        for reverb in self.reverbs.iter_mut() {
            z = reverb.tick(z);
        }
        z
    }
    pub fn tick_nonlinear(&mut self, mut z: Vec2) -> Vec2 {
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
            let z = squash(z);
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

fn main() {
    tone(44100, 44100, 400.0)
        .map(|z| vec2(z.x*0.1, z.y))
        .map(|z| z.pol2cart())
        .collect::<Vec<Vec2>>()
        .save(44100, "outt.wav");

    sweep(44100, 2*44100, 40.0, 6000.0)
        .map(|z| vec2(z.x*0.1, z.y))
        .map(|z| z.pol2cart())
        .collect::<Vec<Vec2>>()
        .save(44100, "outsweep.wav");

    
    // basically placing a bunch of poles?
    // series poles parallel poles or what
    let mut rv1 = ReverbChain::new(vec![

        vec2(0.999, 0.08),
        // vec2(0.999, 0.04),
        vec2(0.9, 0.4),
        vec2(0.69, 0.69),
        vec2(0.7, 0.13),

        vec2(0.9985, 0.005),
        vec2(0.9986, 0.01),
        vec2(0.9987, 0.02),
        vec2(0.9988, 0.04),
        vec2(0.9989, 0.08),
        vec2(0.999, 0.16),
        vec2(0.9987, 0.006),
        vec2(0.9988, 0.0012),
        vec2(0.99, 0.12),
        vec2(0.9, 0.73),
        vec2(0.69, 0.43),
        vec2(0.9, 0.73),

    ]);

    let mut out1 = vec![];
    let s = pulses(32000, 44100 * 8)
        .map(|z| vec2(z.x*0.1, z.y));
    for z in s {
        let z = squash(rv1.tick_nonlinear(z));
        // let z = rv2.tick_nonlinear(z);
        out1.push(z);
    }
    out1.iter_mut().for_each(|z| *z = z.pol2cart());
    out1.save(44100, "outreverb.wav");
}
