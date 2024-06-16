mod util;
use util::*;

pub fn squash(z: Vec2) -> Vec2 {
    vec2(1.0 - (-z.x).exp(), z.y)
}

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
        let zcart = prev.pol2cart() + z.pol2cart();
        let zpol = zcart.cart2pol();
        self.mem = zpol;
        // assert!(zpol.x < 2000.0);
        return zpol.cmul_pol(self.wt);
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
    pub fn new2(w: Vec<(Vec2, Vec2)>) -> Self {
        Self {
            reverbs: w.into_iter().map(|w| Reverb::new2(w.0, w.1)).collect(),
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

pub fn para_exp(file_name: &str, coeffs: Vec<Vec2>) {
    let mut rv1 = ReverbChain::new(coeffs);
    let mut out1 = vec![];
    let s = pulses(32000, 44100 * 8)
        .map(|z| vec2(z.x, z.y));
    for z in s {
        let z = squash(rv1.tick_parallel(z));
        // let z = rv2.tick_nonlinear(z);
        out1.push(z);
    }
    out1.iter_mut().for_each(|z| *z = z.pol2cart());
    out1.save(44100, file_name);
}

pub fn series_exp(file_name: &str, coeffs: Vec<Vec2>) {
    let mut rv1 = ReverbChain::new(coeffs);
    let mut out1 = vec![];
    let s = pulses(32000, 44100 * 8)
        .map(|z| vec2(z.x, z.y));
    for z in s {
        let z = squash(rv1.tick_nonlinear(z));
        // let z = rv2.tick_nonlinear(z);
        out1.push(z);
    }
    out1.iter_mut().for_each(|z| *z = z.pol2cart());
    out1.save(44100, file_name);
}

pub fn series_lin_exp(file_name: &str, coeffs: Vec<Vec2>) {
    let mut rv1 = ReverbChain::new(coeffs);
    let mut out1 = vec![];
    let s = pulses(32000, 44100 * 8)
        .map(|z| vec2(z.x, z.y));
    for z in s {
        let z = rv1.tick(z);
        // let z = rv2.tick_nonlinear(z);
        out1.push(squash(z));
    }
    out1.iter_mut().for_each(|z| *z = z.pol2cart());
    out1.save(44100, file_name);
}

pub fn steppy_exp(file_name: &str, coeffs: Vec<Vec2>) {
    let mut rv1 = ReverbChain::new(coeffs);
    let mut out1 = vec![];
    let s = duty_square(12000, 12000, 44100 * 8)
        .map(|z| vec2(z.x, z.y));
    for z in s {
        let z = rv1.tick(z);
        // let z = rv2.tick_nonlinear(z);
        out1.push(squash(z));
    }
    out1.iter_mut().for_each(|z| *z = z.pol2cart());
    out1.save(44100, file_name);
}

pub fn steppy_para(file_name: &str, coeffs: Vec<Vec2>) {
    let mut rv1 = ReverbChain::new(coeffs);
    let mut out1 = vec![];
    let s = duty_square(12000, 12000, 44100 * 8)
        .map(|z| vec2(z.x, z.y));
    for z in s {
        let z = rv1.tick_parallel(z);
        // let z = rv2.tick_nonlinear(z);
        out1.push(squash(z));
    }
    out1.iter_mut().for_each(|z| *z = z.pol2cart());
    out1.save(44100, file_name);
}

pub fn steppy_para2(file_name: &str, coeffs: Vec<(Vec2, Vec2)>) {
    let mut rv1 = ReverbChain::new2(coeffs);
    let mut out1 = vec![];
    let s = duty_square(12000, 0, 44100 * 8)
        .map(|z| vec2(z.x, z.y));
    for z in s {
        let z = rv1.tick_parallel(z);
        // let z = rv2.tick_nonlinear(z);
        out1.push(squash(z));
    }
    out1.iter_mut().for_each(|z| *z = z.pol2cart());
    out1.save(44100, file_name);
}

fn main() {
    series_exp("outreverb.wav", vec![
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

    series_lin_exp("s_octs.wav", vec![
        // vec2(0.999, 0.0), // idk why this doesnt work tho.
        vec2(0.999, 0.0025),
        vec2(0.999, 0.005),
        vec2(0.999, 0.01),
        vec2(0.999, 0.02),
        vec2(0.999, 0.04),
        // vec2(0.999, 0.08),
        // vec2(0.999, 0.16),
        // ha ha the compression is cooked
    ]);

    steppy_exp("square.wav", vec![
        vec2(0.9995, 0.16),
        vec2(0.9995, 0.32),

        vec2(0.9, 0.04),
        vec2(0.9, 0.08),
        vec2(0.9, 0.16),
        vec2(0.999, 0.0025),
        // vec2(0.999, 0.004),
        vec2(0.999, 0.005),
        vec2(0.999, 0.01),
        vec2(0.999, 0.02),
        vec2(0.999, 0.04),
    ]);
    steppy_para("modes.wav", vec![
        vec2(0.999, 0.01),
        vec2(0.999, 0.02),
        vec2(0.9999, 0.04),
    ]);
    steppy_para2("modes2.wav", vec![
        // (vec2(0.9999, 0.005), vec2(0.1, 0.0)),
        // (vec2(0.9999, 0.01), vec2(0.5, 0.0)),
        // (vec2(0.9999, 0.02), vec2(0.25, 0.0)),
        // (vec2(0.9999, 0.04), vec2(0.125, 0.0)),
        (vec2(0.9999, 0.05), vec2(1.0, 0.0)),
    ]);
    para_exp("para.wav", vec![
        // vec2(0.999, 0.025),
        vec2(0.999, 0.005),
        vec2(0.999, 0.0025),
        // needs just some amplitude mixing
    ])


}
