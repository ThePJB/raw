mod reverb;
use reverb::*;
mod util;
use util::*;

pub const FS: f64 = 48000.0;
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

    click_point: Vec2,
}
impl Instrument {
    pub fn new(gl: glow::Context) -> Self {
        let mut inst = Instrument {
            rc: RenderContext::new(gl),
            sc: SoundContext::new(None, vec![]),
            reverb: ReverbChain::new(vec![
                // vec2(0.999, 0.01),
                // vec2(0.999, 0.02),
                vec2(0.99999, 0.005),
                vec2(0.999, 0.01),
                vec2(0.999, 0.02),
                vec2(0.99, 0.23),
                vec2(0.99, 0.22),
                vec2(0.99, 0.44),
            ]),
            n: 0,
            t: 0.0,
            rng: random_seed(),
            click_point: vec2(0.5, 0.5),
            harm: 1.0,
            mag: 0.0,
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

        // let click = if input.click_lmb { 1.0 } else { 0.0 };
        if input.held_lmb {
            self.click_point = mouse;
            self.harm = mouse.x*TAU;
            self.mag = self.click_point.y;
        } else {
            self.mag *= 0.99;
            // self.mag = 0.0;
        }
        /*
         * Send Audio
         */
        let e_sec = input.elapsed_time.as_secs_f64();
        let e_samp = (e_sec * FS) as i64;

        let mut buf = Vec::new();

        for i in 0..e_samp {
            let w = self.sample();
            buf.push(w.x);
            buf.push(w.y);
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