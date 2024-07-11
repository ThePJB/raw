mod util;
use crate::util::*;

// am notes
// what about an AM architecture tho
// it kind of subtractive.
// starts with dc 1. signal
// times by some envelopy functions
// add some harmonics
// wait is this equal to ring modulation?
// i imagine this parallel. but can it be series too? Or is it the same, its commutative operation
// apparently AM vs RM is whether domain is 0..1 or -1..1 i think.
// its actually modulation index. mod index of 1 is ring mod

pub const FS: f32 = 44100.0;

// nb likey code patch structure.
// this is functional though, for fm and stuff it needs state. Maybe with a closure-generator eh
pub fn am_patch(t: f32) -> f32 {
    (TAU*t*0.1).sin() *
    (1.0 - 0.5*(TAU*t*1.0).sin()) *
    (TAU*t*10.0).sin() *
    (TAU*t*100.0).sin() *
    (
        (TAU*t*220.0).sin() +
        (TAU*t*330.0).sin() +
        (TAU*t*440.0).sin() +
        (TAU*t*550.0).sin() +
        (TAU*t*660.0).sin() +
        (TAU*t*770.0).sin() +
        (TAU*t*880.0).sin()
    )
}

pub fn am_patch2(t: f32) -> f32 {
    (TAU*t*0.1).sin() *
    (TAU*t*1.0).sin() *
    (TAU*t*10.0).sin() *
    (TAU*t*100.0).sin() *
    (TAU*t*1000.0).sin()
}

pub fn main() {
    let mut out1 = vec![];
    for i in 0..244100 {
        let t = i as f32 / FS;
        let x = am_patch(t);
        out1.push(0.1 * vec2(x, x));
    }
    out1.save(44100, "am_test.wav");
}