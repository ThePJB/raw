use crate::impl_vec;
use super::*;

#[derive(Clone, Copy, Debug, Eq, Hash, Default)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

pub const fn ivec2(x: i32, y: i32) -> IVec2 {
    IVec2 { x, y }
}

pub trait AsVec2 {
    fn as_vec2(&self) -> Vec2;
}

impl AsVec2 for IVec2 {
    fn as_vec2(&self) -> Vec2 {
        vec2(self.x as f32, self.y as f32)
    }
}

impl_vec!(IVec2, i32, x, y);

// vecs refactor, colour is ivec4 of u8s, etc
