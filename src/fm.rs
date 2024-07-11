mod util;
use crate::util::*;

pub const FS: f32 = 44100.0;

#[derive(Default)]
pub struct Oscillator {
    phase: f32,
}
impl Oscillator {
    pub fn tick(&mut self, dphase: f32) -> f32 {
        self.phase += dphase;
        self.phase.sin()
    }
}

pub struct Operator {
    osc: Oscillator,
    f: f32, // base frequency
    i: f32, // index of modulation
}
impl Operator {
    // assume f ratio -1 .. 1
    // i have no idea if this is correct
    pub fn tick(&mut self, f_ratio: f32) -> f32 {
        self.osc.tick(self.f*(1.0 + self.i*f_ratio/2.)*TAU/FS)
    }
    pub fn new(i: f32, f: f32) -> Self {
        Self {
            osc: Oscillator::default(),
            f,
            i,
        }
    }
}
pub struct OperatorAM {
    osc: Oscillator,
    f: f32, // base frequency
    i: f32, // index of modulation
}
impl OperatorAM {
    // assume f ratio -1 .. 1
    // i have no idea if this is correct
    pub fn tick(&mut self, f_ratio: f32) -> f32 {
        self.osc.tick(self.f*(1.0 + self.i*f_ratio/2.)*TAU/FS)
    }
    pub fn new(i: f32, f: f32) -> Self {
        Self {
            osc: Oscillator::default(),
            f,
            i,
        }
    }
}

// its more like series patch
pub struct LinearPatch {
    ops: Vec<Operator>,
}
impl LinearPatch {
    pub fn tick(&mut self) -> f32 {
        let mut x = 1.0;
        for op in self.ops.iter_mut() {
            x = op.tick(x);
        }
        x
    }
    pub fn new(params: Vec<(f32, f32)>) -> Self {
        Self {
            ops: params.into_iter().map(|(i, f)| Operator::new(i, f)).collect()
        }
    }
}
// its more like series patch
pub struct ParallelPatch {
    ops: Vec<Operator>,
}
impl ParallelPatch {
    pub fn tick(&mut self) -> f32 {
        let mut x = 1.0;
        let mut acc = 0.0;
        for op in self.ops.iter_mut() {
            acc += op.tick(x);
        }
        acc
    }
    pub fn new(params: Vec<(f32, f32)>) -> Self {
        Self {
            ops: params.into_iter().map(|(i, f)| Operator::new(i, f)).collect()
        }
    }
}

// oi make a parallel fm architecture only as well, see what that can make tho

// read chowning fm paper tho

// u like kinda can do an enevelope lol

// for inst maybe pass in fundamental etc
// adsr and like curves for shit would be good
// parallel would be useful too

// matrix patch would need like topological sort to do the order...
// or could bring in series patch and parallel patch with box<series patch> etc in it.
// box dyn ticker maybe. Series and Parallel both implement ticker as do 

// more useful for sfx will be like a curve editor
// but yea i think u can technically make anything with this if it had starting phase as well - fourier theorem

// granular would be cool especially if you draw the wave shape
// grain interpolation bra
// formant synthesis

// eh current formulation of parallel patch doesnt work

// one idea is can the patch be code or no. functions of &mut x. stuff +. etc
// fn patch_a(x: &mut f32)
// or is it better if it returns shit tho. patch is f32 -> f32
pub fn am_patch(t: f32) -> f32 {
    (TAU*t*1.0).sin() *
    (TAU*t*440.0).sin()
}


pub fn main() {
    let mut p = LinearPatch::new(vec![
        (1.0, 10.),
        (1.0, 11.),
        (1.0, 12.),
        (1.0, 13.),
        (1.0, 61.),
        (1.0, 440.0),
    ]);
    // let mut p = LinearPatch::new(vec![
    //     (1.0, 2.5),
    //     (1.0, 880.0),
    //     (1.0, 440.0),
    //     (1.0, 880.0),
    //     (1.0, 440.0),
    // ]);
    // let mut p = LinearPatch::new(vec![
    //     (1.0, 880.0),
    //     (1.0, 440.0),
    //     (0.5, 220.0),
    //     (1.0, 880.0),
    //     (1.0, 880.0),
    //     (1.0, 440.0),
    //     (0.5, 220.0),
    //     (1.0, 880.0),
    //     (1.0, 880.0),
    //     (1.0, 440.0),
    //     (0.5, 220.0),
    //     (1.0, 880.0),
    //     (1.0, 880.0),
    //     (1.0, 440.0),
    //     (0.5, 220.0),
    //     (1.0, 880.0),
    // ]);

    let mut out1 = vec![];
    for _i in 0..144100 {
        let x = p.tick();
        out1.push(0.1 * vec2(x, x));
    }
    out1.save(44100, "fm_test.wav");
}

pub fn main3() {
    let mut op = Operator::new(0.5, 880.0);

    let mut out1 = vec![];
    for _i in 0..144100 {
        let x = op.tick(1.0);
        out1.push(0.1 * vec2(x, x));
    }
    out1.save(44100, "fm_test.wav");
}

pub fn main2() {
    let fs = 44100.;
    let mut out1 = vec![];
    let mut phase = 0.0;
    // let fmr = 1./(2.0f32).sqrt();
    let fmr = 0.01 / PHI;
    let fc = 440.0;
    let depth = 1.0;
    for i in 0..144100 {
        let fm_phase = TAU * fc * fmr * (1. / fs) * i as f32;
        let dphase = fc + fc * depth * fm_phase.sin();
        let dphase = dphase * (TAU/fs);
        // let dphase = TAU/fs * fc + (TAU * 0.5 * fc * i as f32/fs).sin();
        // let dphase = 440.0f32 * (1. / fs) * TAU;
        phase += dphase;
        let x = phase.sin();
        out1.push(0.1 * vec2(x, x));
    }
    out1.save(44100, "fm_test.wav");
}