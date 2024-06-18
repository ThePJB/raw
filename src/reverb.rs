use crate::util::*;

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
    pub fn tick_nonlinear_xc(&mut self, mut z: Vec2) -> Vec2 {
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