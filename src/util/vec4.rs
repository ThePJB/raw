use crate::impl_vec;
use std::cmp::{PartialEq, Eq};

#[derive(Clone, Copy, Debug, Default)]
#[repr(C, packed)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub const fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4 { x, y, z, w }
}

impl_vec!(Vec4, f32, x, y, z, w);