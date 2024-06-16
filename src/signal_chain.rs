use crate::util::*;

pub fn squash(z: Vec2) -> Vec2 {
    vec2(1.0 - (-z.x).exp(), z.y)
}

pub trait SignalChain {
    fn tick(&mut self, z: Vec2) -> Vec2;
}
pub struct Series {
    elems: Vec<Box<dyn SignalChain>>
}
impl SignalChain for Series {
    fn tick(&mut self, mut z: Vec2) -> Vec2 {
        for elem in self.elems.iter_mut() {
            z = elem.tick(z);
        }
        z
    }
}
impl Series {
    pub fn new(elems: Vec<Box<dyn SignalChain>>) -> Box<dyn SignalChain> {
        return Box::new(Self{elems})
    }
}
pub struct Parallel {
    elems: Vec<Box<dyn SignalChain>>
}
impl SignalChain for Parallel {
    fn tick(&mut self, z: Vec2) -> Vec2 {
        let mut acc = vec2(0.0, 0.0);
        for elem in self.elems.iter_mut() {
            let z = elem.tick(z);
            acc = acc.cadd_pol(z);
        }
        acc
    }
}
impl Parallel {
    pub fn new(elems: Vec<Box<dyn SignalChain>>) -> Box<dyn SignalChain> {
        return Box::new(Self{elems})
    }
}
pub struct Feedback {
    w: Vec2,
    mem: Vec2,
    elem: Box<dyn SignalChain>,
}
impl SignalChain for Feedback {
    fn tick(&mut self, z: Vec2) -> Vec2 {
        self.mem = self.elem.tick(self.mem.cadd_pol(z.cmul(self.w)));
        self.mem
    }
}
impl Feedback {
    pub fn new(elem: Box<dyn SignalChain>, w: Vec2) -> Box<Self> {
        return Box::new(Self{elem,w,mem:vec2(0.0, 0.0)})
    }
}
impl SignalChain for () {
    fn tick(&mut self, z: Vec2) -> Vec2 {
        z
    }
}
pub struct ComplexReverb {
    w: Vec2,
    mem: Vec2,
}
impl SignalChain for ComplexReverb {
    #[track_caller]
    fn tick(&mut self, z: Vec2) -> Vec2 {
        let prev = self.mem.cmul_pol(self.w);
        let mut z = z.cadd_pol(prev);
        // z.x /= 2.0;
        // let z = squash(z);
        self.mem = z;
        assert!(z.x < 2000.0);
        return z;
    }
}
impl ComplexReverb {
    pub fn new(w: Vec2) -> Box<dyn SignalChain> {
        Box::new(Self {
            w,
            mem: vec2(0.0, 0.0),
        })
    }
}
pub fn reverb_chain(w: Vec2, n: usize) -> Box<dyn SignalChain> {
    Series::new(
        repeat(w).map(|w| ComplexReverb::new(w)).take(n).collect(),
    )
}

pub fn run(name: &str, mut elem: Box<dyn SignalChain>, input: impl Iterator<Item = Vec2>, gain: Vec2) {
    let mut out = vec![];
    for z in input {
        out.push(squash(elem.tick(z).cmul_pol(gain)))
    }
    out.iter_mut().for_each(|z| *z = z.pol2cart());
    out.save(44100, name)
}