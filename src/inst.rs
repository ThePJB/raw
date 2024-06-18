mod reverb;
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
}
impl Instrument {
    pub fn new(gl: glow::Context) -> Self {
        let mut inst = Instrument {
            rc: RenderContext::new(gl),
            sc: SoundContext::new(None, vec![]),
            n: 0,
            t: 0.0,
        };
        inst.initialize();
        inst
    }
    pub fn initialize(&mut self) {
        self.sc.send_samples(repeat(0.0).take(4000));
    }
}
impl App for Instrument {
    fn frame(&mut self, input: Input) {
        /*
         * Update states
         */


        /*
         * Send Audio
         */
        let e_sec = input.elapsed_time.as_secs_f64();
        let e_samp = (e_sec * FS) as i64;
        // sample of t aye
        let samples = (0..(2*e_samp))
            .map(|n| n/2) // channels
            .map(|n| self.t + n as f64 * TS)    // sample number to time
            .map(|t| (440.0*TAU*t as f32).sin())   // time to amplitude
            .map(|x| x * 0.01); // volume
        self.sc.send_samples(samples);
        self.n += e_samp;
        self.t += e_sec;

        /*
         * Draw UI
         */

        

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