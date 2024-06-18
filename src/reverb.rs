use crate::util::*;

pub fn squash(z: Vec2) -> Vec2 {
    vec2(1.0 - (-z.x).exp(), z.y)
}

pub struct Reverb {
    mem: Vec2,
    w: Vec2,
}
impl Reverb {
    pub fn new(w: Vec2) -> Self {
        Self {
            mem: vec2(0.0, 0.0),
            w,
        }
    }
    pub fn tick(&mut self, z: Vec2, harm: f32) -> Vec2 {
        let mut w = self.w;
        w.y *= harm;
        let prev = self.mem.cmul_pol(w);
        let zcart = prev.pol2cart() + z.pol2cart();
        let zpol = zcart.cart2pol();
        self.mem = zpol;
        return zpol.cmul_pol(w);
    }
}
pub struct ReverbChain {
    pub reverbs: Vec<Reverb>,
}
impl Default for ReverbChain {
    fn default() -> Self {
        Self {
            reverbs: vec![],
        }
    }
}
impl ReverbChain {
    pub fn new(w: Vec<Vec2>) -> Self {
        Self {
            reverbs: w.into_iter().map(|w| Reverb::new(w)).collect(),
            ..Default::default()
        }
    }
    pub fn tick(&mut self, mut z: Vec2, harm: f32) -> Vec2 {
        for reverb in self.reverbs.iter_mut() {
            z = reverb.tick(z, harm);
            z = vec2(1.0 - (-z.x).exp(), z.y);
        }
        z = vec2(1.0 - (-z.x).exp(), z.y);
        z
    }

}