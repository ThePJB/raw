use crate::impl_vec;
use std::cmp::{PartialEq, Eq};
use super::*;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C, packed)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub const fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

impl Vec2 {
    pub fn projx(&self) -> Vec2 {
        vec2(self.x, 0.0)
    }
    pub fn projy(&self) -> Vec2 {
        vec2(0.0, self.y)
    }
    pub fn rotate(&self, theta: f32) -> Vec2 {
        let c = theta.cos();
        let s = theta.sin();
        let c1 = vec2(c, s);
        let c2 = vec2(-s, c);
        vec2(c1.dot(&self), c2.dot(&self))
    }
    pub fn extend(&self, z: f32) -> Vec3 {
        vec3(self.x, self.y, z)
    }
    pub fn yx(&self) -> Vec2 {
        vec2(self.y, self.x)
    }
    pub fn cmul(&self, other: Self) -> Self {
        vec2(self.x * other.x - self.y * other.y, self.y * other.x + other.y * self.x)
    }
    pub fn cmul_pol(&self, other: Self) -> Self {
        vec2(self.x*other.x, self.y + other.y)
    }
    pub fn cart2pol(&self) -> Self {
        vec2(self.dot(self).sqrt(), self.y.atan2(self.x))
    }
    pub fn pol2cart(&self) -> Self {
        self.x * vec2(self.y.cos(), self.y.sin())
    }
}


impl_vec!(Vec2, f32, x, y);