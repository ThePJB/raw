mod util;
use util::*;
mod signal_chain;
use signal_chain::*;

pub fn pulses(t: usize, n: usize, amp: f32) -> impl Iterator<Item = Vec2> {
    (0..n).map(move |i| {
        let m = i % t == 0;
        let mf = if m { amp } else { 0.0 };
        vec2(mf, i as f32)
    })
}

pub fn main() {
    run(
        "sigchain.wav", 
        ComplexReverb::new(vec2(0.999, 0.16)),
        pulses(8000, 44100*4, 0.01),
        vec2(1.0, 0.0),
    );

    run (
        "sigseries.wav",
        Series::new(
                vec![
                    ComplexReverb::new(vec2(0.999, 0.0125)),
                    ComplexReverb::new(vec2(0.999, 0.05)),
                    ComplexReverb::new(vec2(0.999, 0.025)),
                ]
        ),
        pulses(8000, 44100*4, 0.1),
        vec2(1.0, 0.0),
    );

    // run (
    //     "sigseries2.wav",
    //     Series::new(
    //             vec![
    //                 ComplexReverb::new(vec2(0.999, 0.08)),
    //                 ComplexReverb::new(vec2(0.999, 0.04)),
    //                 ComplexReverb::new(vec2(0.9, 0.4)),
    //                 ComplexReverb::new(vec2(0.69, 0.69)),
    //                 ComplexReverb::new(vec2(0.7, 0.13)),
    //                 ComplexReverb::new(vec2(0.9985, 0.005)),
    //                 ComplexReverb::new(vec2(0.9986, 0.01)),
    //                 ComplexReverb::new(vec2(0.9987, 0.02)),
    //                 ComplexReverb::new(vec2(0.9988, 0.04)),
    //                 ComplexReverb::new(vec2(0.9989, 0.08)),
    //                 ComplexReverb::new(vec2(0.999, 0.16)),
    //                 ComplexReverb::new(vec2(0.9987, 0.006)),
    //                 ComplexReverb::new(vec2(0.9988, 0.0012)),
    //                 ComplexReverb::new(vec2(0.99, 0.12)),
    //                 ComplexReverb::new(vec2(0.9, 0.73)),
    //                 ComplexReverb::new(vec2(0.69, 0.43)),
    //                 ComplexReverb::new(vec2(0.9, 0.73)), 
    //             ]
    //     ),
    //     pulses(8000*4, 44100*4, 0.1),
    //     vec2(0.1, 0.0),
    // );

    run (
        "wg3re.wav",
        reverb_chain(vec2(0.9, 0.01), 5),
        pulses(8000*4, 44100*4, 0.1),
        vec2(1.0, 0.0),
    );
}