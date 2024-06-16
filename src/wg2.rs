mod util;
use util::*;
use std::{collections::VecDeque, iter::repeat, iter::once, iter::repeat_with};

pub fn squash(z: Vec2) -> Vec2 {
    vec2(1.0 - (-z.x).exp(), z.y)
}

pub struct Waveguide {
    mem: VecDeque<Vec2>,
    w_pol: Vec2,
}
impl Waveguide {    
    pub fn new(damping: f32, c: f32, f: f32) -> Self {
        let fs = 44100;
        let l = fs as f32/(f*2.0);
        dbg!(l); // 38000 for a 400hz tone! that doesnt make sense
        let residual_phase = l.fract() * TAU;
        let magnitude = (-damping).exp();
        Self {
            mem: VecDeque::from_iter(repeat(vec2(0.0, 0.0)).take(l.floor() as usize)),
            w_pol: vec2(magnitude, residual_phase),
        }
    }

    pub fn tick(&mut self, z: Vec2) -> Vec2 {
        // maybe everything has infinite input impedance digitally./ not sure if good
        // it like takes time to propagate though
        // needs like forward and back maybe
        // can we peek the 
        let peek = self.mem.pop_front().unwrap().cmul_pol(self.w_pol);
        let z = z.cadd_pol(peek);
        self.mem.push_back(z);
        return z;
    }
}
pub struct Series {
    elems: Vec<Waveguide>,
}
impl Series {
    pub fn new(elems: Vec<(f32, f32, f32)>) -> Self {
        Self {
            elems: elems.into_iter().map(|(damping, c, f)| Waveguide::new(damping, c, f)).collect(),
        }
    }
    pub fn tick(&mut self, mut z: Vec2) -> Vec2 {
        for elem in self.elems.iter_mut() {
            z = elem.tick(z);
        }
        z
    }
}
pub struct Parallel {
    elems: Vec<Waveguide>,
}
impl Parallel {
    pub fn new(elems: Vec<(f32, f32, f32)>) -> Self {
        Self {
            elems: elems.into_iter().map(|(damping, c, f)| Waveguide::new(damping, c, f)).collect(),
        }
    }
    pub fn tick(&mut self, z: Vec2) -> Vec2 {
        let mut acc = vec2(0.0, 0.0);
        for elem in self.elems.iter_mut() {
            let z = elem.tick(z);
            acc += z;
        }
        acc
    }
}

// todo waveguide is a trait and its box dyn waveguide
// and we can have serieses of parallels and vice versa


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
pub fn noise(n: usize) -> impl Iterator<Item = Vec2> {
    let mut rng = random_seed();
    repeat_with(move || vec2(rng.next_f32(), rng.next_f32()*TAU)).take(n)
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

pub fn para_exp(file_name: &str, coeffs: Vec<(f32, f32, f32)>) {
    let mut rv1 = Parallel::new(coeffs);
    let mut out1 = vec![];
    // let s = duty_square(500, 32000, 44100 * 8)
    //     .map(|z| vec2(z.x * 0.1, z.y));
    // let s = once(vec2(0.1, 0.0)).chain(repeat(vec2(0.1, 0.0)).take(44100*8));
    // let s1 = sweep(44100, 44100 * 8, 20.0, 20000.0)
    //     .map(|x| vec2(x.x*0.01, x.y));
    // let s2 = sweep(44100, 44100 * 8, 20000.0, 20.0)
    //     .map(|x| vec2(x.x*0.01, x.y)).into_iter();
    // let s = s1.chain(s2);
    let s = noise(44100*8).map(|x| vec2(x.x*0.0001, x.y));
    for z in s {
        let z = rv1.tick(z);
        // let z = rv2.tick_nonlinear(z);
        out1.push(squash(z));
    }
    out1.iter_mut().for_each(|z| *z = z.pol2cart());
    out1.save(44100, file_name);
}

fn main() {
    // what if magnitudes are in db ie stored as log magnitude. Is that even more natural?
    // then u can add them or something
    // but oi whats goin on with this where repeat 1s basically isnt resonating
    // bra that phase shit kinda worked better tho
    // it was doing less cause it was like a perfect wavelength transformer or whatever
    // todo remove speed
    // maybe speed is uniquely determined by resonant freuqency? well it should have L as well no. maybe theyre coupled in digital
    // bra that 
    // gaddam why is it so bad. im not really sure but the sweep as ok.
    para_exp("wg2test.wav", vec![
    //    (100.0, 346.0, 20000.0), // yea i wanted just 1 tap only basically to smoooth er out
        // maybe all 1s not resonating is expected
       (0.0001, 346.0, 50.0),
    ]);


}
