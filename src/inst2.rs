mod reverb;
use std::time::Instant;

use reverb::*;
mod util;
use util::*;

pub const FS: f64 = 48000.0;
pub const TS: f64 = 1.0 / FS;

pub struct Instrument {
    rc: RenderContext,
    sc: SoundContext,
    n: i64,
    t: f64,
    rng: u32,
    phase: f32,

    t_send: std::time::Instant,

    click_point: Vec2,
    mouse_held: f32,
}
impl Instrument {
    pub fn new(gl: glow::Context) -> Self {
        let mut inst = Instrument {
            rc: RenderContext::new(gl),
            sc: SoundContext::new(None, vec![]),
            n: 0,
            t: 0.0,
            phase: 0.0,
            rng: random_seed(),
            click_point: vec2(0.5, 0.5),
            t_send: Instant::now(),
            mouse_held: 0.0,
        };
        inst.initialize();
        inst
    }
    pub fn initialize(&mut self) {
        self.sc.send_samples(repeat(0.0).take(4000));
    }

    // x and y real stereo sample
    pub fn sample(&mut self) -> Vec2 {
        self.n += 1;
        self.t += TS;

        let x = self.click_point.x;
        // let x = (1. + x * 12.).floor() / 12.;
        let mag = self.click_point.y;

        

        // let freq = (x*10.0).exp2();
        // let period = 0.02*(1.0 - x); // does this make sonic frequencies?
        // let period = 1./x;
        let w = 0.05*x;
        self.phase += w; // ctrl to freq out instead of vol out
        self.phase = self.phase.fract();
        // ahh .5 seconds is 2 hz.
        // triangle wave with that as the period
        // let gen = 2. * (2. * ((self.t as f32 % period)/period).abs() - 1.) - 1.;
        let gen = 2. * (2.*self.phase - 1.).abs() - 1.;
        let gen = gen * mag;
        // now compose this into odd -1..1 wavelets

        // might not rly be that shape but
        // operation that is like 
        //let gen = gen.signum() + 0.2*(gen*8.*PI).sin();
        let gen = (gen * 10.).tanh(); // yung tanh. yea what about tanh but that it like wraps around too

        // let gen = gen.signum();
        // let gen = (PI*gen).sin();

        let stereo = vec2(gen, gen);
        stereo*0.1*self.mouse_held
    }
}
impl App for Instrument {
    fn frame(&mut self, input: Input) {
        /*
         * Update states
         */
        let mouse = input.mouse_px/self.rc.wh.as_vec2();
        if input.held_lmb {
            self.click_point = mouse;
            self.mouse_held = 1.0;
        } else {
            self.mouse_held *= 0.9;
        }
        /*
         * Send Audio
         */
        let t_now = std::time::Instant::now();
        let mut buf = Vec::new();

        while self.t_send < t_now {
            let w = self.sample();
            buf.push(w.x);
            buf.push(w.y);
            self.t_send += std::time::Duration::from_secs_f64(1.0/FS);
        }
        self.sc.send_samples(buf.into_iter());
    }
}

// yea nice sending sine
// lets send that bass synth
// and clicking and ui etc

fn main() {
    let sc = SoundContext::new(None, vec![]);
    let context = Context::new("Imaginary Instrument");
    let gl = context.get_gl();
    let inst = Instrument::new(gl);
    context.run(inst);
}