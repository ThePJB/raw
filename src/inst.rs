mod reverb;
use std::time::Instant;

use reverb::*;
mod util;
use util::*;

pub const FS: f64 = 44100.0;
pub const TS: f64 = 1.0 / FS;

pub struct Instrument {
    reverb: ReverbChain,
    rc: RenderContext,
    sc: SoundContext,
    n: i64,
    t: f64,
    harm: f32,
    mag: f32,
    rng: u32,

    t_send: std::time::Instant,

    click_point: Vec2,
}
impl Instrument {
    pub fn new(gl: glow::Context) -> Self {
        let mut inst = Instrument {
            rc: RenderContext::new(gl),
            sc: SoundContext::new(None, vec![]),
            reverb: ReverbChain::new(vec![
                vec2(0.9999, 0.02), // fkn bass transient is so good
                vec2(0.9999, 0.04), // fkn bass transient is so good
                // vec2(0.99, 0.012), // fkn bass transient is so good
                // vec2(0.99, 0.013),
                vec2(0.999, 0.06),
                vec2(0.9992, 0.04),
                vec2(0.999, 0.03),
                vec2(0.999, 0.02),
                vec2(0.999, 0.01),
                vec2(0.9, 0.02),
                vec2(0.9, 0.01),
                vec2(0.999, 0.04),
                vec2(0.999, 0.08),
                vec2(0.9, 0.32),
                vec2(0.99, 0.08),

                // satan derived
                vec2(0.99999, 0.004),
                vec2(0.99999, 0.008),
                vec2(0.99, 0.43),
                vec2(0.99, 0.65),
                vec2(0.9999, 0.016),
                vec2(0.9999, 0.0326),
                vec2(0.99999, 0.0647),
                vec2(0.9999, 0.1288),
                vec2(0.999, 0.016),
                vec2(0.99, 0.2569),

                vec2(0.9999, 0.02),
                vec2(0.9999, 0.03),
                vec2(0.9999, 0.04),
                vec2(0.999, 0.05),

                vec2(0.99, 0.012),
                vec2(0.99, 0.013),
                vec2(0.998, 0.081),
                vec2(0.997, 0.082),
                vec2(0.999, 0.04),
                vec2(0.9994, 0.02),
                vec2(0.9998, 0.01),
                vec2(0.99, 0.16),
            ]),
            n: 0,
            t: 0.0,
            rng: random_seed(),
            click_point: vec2(0.5, 0.5),
            harm: 1.0,
            mag: 0.0,
            t_send: Instant::now(),
        };
        inst.initialize();
        inst
    }
    pub fn initialize(&mut self) {
        self.sc.send_samples(repeat(0.0).take(16000));
    }
    // x and y real stereo sample
    pub fn sample(&mut self) -> Vec2 {
        self.n += 1;
        self.t += TS;
        // let z = self.reverb.tick(vec2(self.mag, 0.0), self.harm); // maybe this is a good parameter to make random
        let z = self.reverb.tick(vec2(self.mag, self.harm), 1.0); // maybe this is a good parameter to make random
        let z = squash(z);
        let z = z.pol2cart();
        let stereo = vec2(z.x,z.y);
        stereo*0.1
    }
}
impl App for Instrument {
    fn frame(&mut self, input: Input) {
        /*
         * Update states
         */
        let mouse = input.mouse_px/self.rc.wh.as_vec2();

        // hang on it was meant to be the pitch of the first oscillator too
        // oh yeah a first reverb not really or a first reverb state

        self.mag = if input.click_lmb { 1.0 } else { 0.0 };
        if input.held_lmb {
            self.click_point = mouse;
            self.harm = mouse.x*4.0;
            // self.mag = self.click_point.y;
        } else {
            // self.mag *= 0.99;
            self.mag = 0.0;
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