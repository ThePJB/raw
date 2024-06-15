use crate::impl_vec;
use std::cmp::{PartialEq, Eq};

#[derive(Clone, Copy, Debug, Default)]
#[repr(C, packed)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub const fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

impl_vec!(Vec3, f32, x, y, z);