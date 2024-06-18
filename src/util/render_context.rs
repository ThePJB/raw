use glow::Texture as GPUTexture;
use glow::Context as GLContext;
use glow::*;
use glutin::Rect;
use std::{collections::HashMap, f32::consts::PI};

use super::*;
use crate::util::*;


/*
Gonna construct lots of these
*/
#[derive(Clone)]
pub struct RenderCommand {
    pub pos: Vec3, // xyz, -z means more forward and also want to impl partialord on z
    pub wh: Vec2,   // might be some shit where wh 0 means get from asset
    pub colour: Vec4,
    pub asset_name: String,
    pub center_x: bool,
    pub center_y: bool,
    pub uv_divs: IVec2,
    pub uv_inds: IVec2,
}

impl RenderCommand {
    pub fn draw_albedo(&self, buf: &mut VertexBufCPU, render_context: &RenderContext) {
        // todo do this
        let h = render_context.resource_handles[&self.asset_name];
        let sprite_wh = h.wh.as_vec2() / ATLAS_WH.as_vec2();
        let sprite_xy = h.xy.as_vec2() / ATLAS_WH.as_vec2();
        let sprite_xy = sprite_xy + self.uv_inds.as_vec2() * sprite_wh / self.uv_divs.as_vec2();
        let sprite_wh = sprite_wh / self.uv_divs.as_vec2();
        let uvs = [vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(1.0, 1.0), vec2(0.0, 1.0)];
        let wh = if self.wh.x == 0.0 && self.wh.y == 0.0 {
            sprite_wh
        } else {
            self.wh
        };
        let verts = (0..4)
            .map(|i| uvs[i])
            .map(|uv| {
                let p = vec2(self.pos.x, self.pos.y) + wh * uv;
                let uv = sprite_xy + sprite_wh * uv;
            Vertex {
                xyz: vec3(p.x, p.y, self.pos.z),
                rgba: self.colour,
                uv: uv,
            }
        });
        let inds = [0, 1, 2, 0, 2, 3].into_iter();
        buf.extend(verts, inds);
    }
}

// pass simplest things first easiest readability i think, less context storing
// guard pattern does this too
// minimize dependency range
pub fn render_text(buf: &mut Vec<RenderCommand>, s: String, mut base: RenderCommand) {
    base.asset_name = "font".to_owned();
    let indiv = base.clone();
    let chars = s.chars();
    let len = s.len();
    let pos_ascii = chars.enumerate()
        .filter_map(|(i, c)| {
            if !c.is_ascii() {
                return None;
            }
            let c = c as u8;
            if c > 0x20 && c < 0x7F {
                return Some((i, c));
            }
            return None;
    });
    pos_ascii.for_each(move |(i, c)| {
        let clip_wh = ivec2(16, 6);
        let ind = c - 0x20;
        let mut xy = vec2(base.pos.x, base.pos.y);
        let char_wh = vec2(8.0, 8.0);
        let text_wh = vec2(8.0 * len as f32, 8.0);
        if base.center_x {
            xy -= text_wh.projx()/2.0;
        }
        if base.center_y {
            xy -= text_wh.projy()/2.0;
        }
        xy += char_wh.projx() * i as f32;
        let clip_xy = ivec2(ind as i32 % clip_wh.x, ind as i32 / clip_wh.x);
        let mut cmd = indiv.clone();
        cmd.pos.x = xy.x;
        cmd.pos.y = xy.y;
        cmd.wh = char_wh;
        cmd.uv_inds = clip_xy;
        cmd.uv_divs = clip_wh;
        buf.push(cmd);
    })
}

impl Default for RenderCommand {
    fn default() -> Self {
        Self {
            pos: vec3(0.0, 0.0, 0.0),
            wh: vec2(0.0, 0.0),
            colour: vec4(1.0, 1.0, 1.0, 1.0),
            asset_name: "debug_square".to_owned(),
            center_x: false,
            center_y: false,
            uv_divs: ivec2(1,1),
            uv_inds: ivec2(0,0),
        }
    }
}
impl PartialEq for RenderCommand {
    fn eq(&self, other: &Self) -> bool {
        let this_z = self.pos.z;
        let other_z = other.pos.z;
        this_z.eq(&other_z)
    }
}
impl PartialOrd for RenderCommand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let this_z = self.pos.z;
        let other_z = other.pos.z;
        this_z.partial_cmp(&other_z)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SpriteHandle {
    pub xy: IVec2,
    pub wh: IVec2,
}

pub const ATLAS_WH: IVec2 = ivec2(1024, 1024);
pub const FRAG_ALBEDO: &str = r#"#version 330 core
in vec4 col;
in vec2 uv;
in vec4 gl_FragCoord;
out vec4 frag_colour;

uniform sampler2D tex;

void main() {
    frag_colour = texture(tex, uv) * col;
}
"#;

pub const VERT_ALBEDO: &str = r#"#version 330 core
layout (location = 0) in vec3 in_pos;
layout (location = 1) in vec4 in_col;
layout (location = 2) in vec2 in_uv;

out vec4 col;
out vec2 uv;

// uniform mat4 projection;
const mat4 projection = mat4(1.0, 0.0, 0.0, 0.0,
    0.0, -1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0);


void main() {
    col = in_col;
    uv = in_uv;
    gl_Position = projection * vec4(in_pos, 1.0);
}
"#;

pub struct RenderContext {
    pub gl: GLContext,
    pub program_albedo: NativeProgram,
    pub vao: VertexArray,
    pub vbo: Buffer,
    pub ebo: Buffer,
    pub texture: GPUTexture,
    pub resource_handles: HashMap<String, SpriteHandle>,
    pub wh: IVec2,
}

impl RenderContext {
    pub fn new(gl: GLContext) -> Self {
        unsafe {
            gl.enable(glow::DEPTH_TEST);
            gl.depth_func(glow::LEQUAL);
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
            gl.disable(glow::CULL_FACE);

            let vbo = gl.create_buffer().unwrap();
            let ebo = gl.create_buffer().unwrap();
            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo)); // Bind the EBO
            // let vert_size: usize = std::mem::size_of::<Vertex>();
            let vert_size = 4*9;
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, vert_size as i32, 0);
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(1, 4, glow::FLOAT, false, vert_size as i32, 3 * 4);
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, vert_size as i32, 7 * 4);
            gl.enable_vertex_attrib_array(2);

            // self.atlas = Some(Atlas::new(gl));
            // self.res = Some(Resources::new(&mut self.atlas.unwrap_mut(), gl));

            // albedo 
            let program_albedo = {
                let program_albedo = gl.create_program().expect("Cannot create program");
            
                let vs = gl.create_shader(glow::VERTEX_SHADER).expect("cannot create vertex shader");
                gl.shader_source(vs, VERT_ALBEDO);
                gl.compile_shader(vs);
                if !gl.get_shader_compile_status(vs) {
                    panic!("{}", gl.get_shader_info_log(vs));
                }
                gl.attach_shader(program_albedo, vs);
        
                let fs = gl.create_shader(glow::FRAGMENT_SHADER).expect("cannot create fragment shader");
                gl.shader_source(fs, FRAG_ALBEDO);
                gl.compile_shader(fs);
                if !gl.get_shader_compile_status(fs) {
                    panic!("{}", gl.get_shader_info_log(fs));
                }
                gl.attach_shader(program_albedo, fs);
        
                gl.link_program(program_albedo);
                if !gl.get_program_link_status(program_albedo) {
                    panic!("{}", gl.get_program_info_log(program_albedo));
                }
                gl.detach_shader(program_albedo, fs);
                gl.delete_shader(fs);
                gl.detach_shader(program_albedo, vs);
                gl.delete_shader(vs);
                program_albedo
            };

            gl.active_texture(glow::TEXTURE0);
            let mut im = ImageBuffer::new(ATLAS_WH);
            im.fill(vec4(1.0, 0.0, 1.0, 1.0));
            let texture = gl.create_texture().unwrap();
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            gl.tex_image_2d(
                glow::TEXTURE_2D, 
                0, 
                glow::RGBA as i32, 
                im.wh.x as i32, im.wh.y as i32, 
                0, 
                RGBA, 
                glow::UNSIGNED_BYTE, 
                Some(&im.data)
            );
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);

            gl.generate_mipmap(glow::TEXTURE_2D);

            RenderContext {
                gl,
                program_albedo,
                vao,
                vbo,
                ebo,
                texture,
                resource_handles: HashMap::new(),
                wh: ivec2(640,360),
            }
        }
    }

    pub fn frame(&mut self, render_list: &Vec<RenderCommand>) {
        unsafe {
            // self.gl.viewport(0, 0, self.wh.x, self.wh.y);
            self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT); 
            let mut buf = VertexBufCPU::default();
            render_list.iter().for_each(|rc| rc.draw_albedo(&mut buf, &self));
            unsafe {
                self.gl.use_program(Some(self.program_albedo));
                // so lol technically u dont have to do this but maybe u do
                self.gl.active_texture(glow::TEXTURE0);
                self.gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
                self.gl.uniform_1_i32(self.gl.get_uniform_location(self.program_albedo, "tex").as_ref(), 0);
                self.gl.bind_vertex_array(Some(self.vao));
                self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
                self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
                let num_verts = buf.inds.len();
                self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, buf.verts.as_bytes(), glow::STATIC_DRAW);
                self.gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, buf.inds.as_bytes(), glow::STATIC_DRAW);
                self.gl.draw_elements(
                    glow::TRIANGLES,
                    num_verts as i32,       // number of indices
                    glow::UNSIGNED_INT,   // type of indices
                    0                           // offset
                );
            }
        }
    }

    pub fn resize(&mut self, wh: IVec2) {
        self.wh = wh;
        unsafe {
            self.gl.viewport(0, 0, wh.x, wh.y);
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C, packed)]
pub struct Vertex {
    pub xyz: Vec3,
    pub rgba: Vec4,
    pub uv: Vec2,
    // uv
    // other shit lmao like specular etc
}

#[derive(Default, Debug)]
pub struct VertexBufCPU {
    pub verts: Vec<Vertex>,
    pub inds: Vec<u32>,
}

impl VertexBufCPU {
    pub fn extend(&mut self, verts: impl Iterator<Item = Vertex>, inds: impl Iterator<Item = u32>) {
        let offset = self.verts.len() as u32;
        self.verts.extend(verts);
        self.inds.extend(inds.map(|ind| ind + offset))
    }
}

// #[derive(Debug)]
// pub struct TriangleArgs {
//     pub p: [Vec2; 3],
//     pub z: f32,
//     pub c: Vec4,
// }
