mod vec_macros;
mod lerp;
mod vec2;
mod vec3;
mod vec4;
mod ivec2;
mod option_utils;
mod gen_ref;
mod image;
mod file_utils;
mod texture;
mod distance_field_generation;
mod rng;
mod signal;

mod sound_context;
pub use sound_context::*;
mod render_context;
pub use render_context::*;
mod context;pub use context::*;


pub use vec2::*;
pub use vec3::*;
pub use vec4::*;
pub use ivec2::*;
pub use image::*;
pub use file_utils::*;
pub use option_utils::*;
pub use texture::*;
pub use distance_field_generation::*;
pub use rng::*;
pub use signal::*;

pub use std::f32::consts::PI;
pub const PHI: f32 = 1.618033988749894848204;
pub use std::f32::consts::TAU;
pub use std::iter::repeat;