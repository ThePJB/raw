use crate::util::*;
pub trait AsBytes {
    fn as_bytes(&self) -> &[u8];
}

impl<T> AsBytes for Vec<T> {
    fn as_bytes(&self) -> &[u8] {
        // Get a pointer to the data and calculate the length in bytes
        let ptr = self.as_ptr() as *const u8;
        let len = self.len() * std::mem::size_of::<T>();

        // Convert the pointer and length to a byte slice
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }
}

#[test]
fn test_as_bytes() {
    let a = vec![1i32, 1000i32, 0i32, 1i32];
    let b = a.as_bytes();
    dbg!(b);
}
pub struct Texture<T: Clone> {
    pub wh: IVec2,
    pub data: Vec<T>,
}

impl<T: Clone + Default> Texture<T> {
    pub fn new(wh: IVec2, initial: T) -> Self {
        Self {
            wh,
            data: vec![initial; (wh.x * wh.y) as usize],
        }
    }

    pub fn fill(&mut self, value: T) {
        for i in 0..self.wh.x {
            for j in 0..self.wh.y {
                self.set(ivec2(i, j), value.clone());
            }
        }
    }

    pub fn sample(&self, coord: IVec2) -> T {
        self.data.get((coord.x + coord.y * self.wh.x) as usize).cloned().unwrap()
    }
    pub fn sample_checked(&self, coord: IVec2) -> Option<T> {
        self.data.get((coord.x + coord.y * self.wh.x) as usize).cloned()
    }

    pub fn save(&self, path: &std::path::Path, f: impl Fn(T)->[u8;4]) {
        use std::fs::File;
        use std::io::BufWriter;

        let file = File::create(path).unwrap();

        let data: Vec<u8> = self.data.iter().map(|px| f(px.clone())).flatten().collect();

        let ref mut buf_writer = BufWriter::new(file);

        let mut encoder = png::Encoder::new(buf_writer, self.wh.x as u32, self.wh.y as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        // encoder.set_trns(vec!(0xFFu8, 0xFFu8, 0xFFu8)); // maybe dont need lol
        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
        encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
        let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000)
        );
        encoder.set_source_chromaticities(source_chromaticities);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&data).unwrap(); // Save
    }

    // could clamp to edge repeat etc
    // float sample, float uv sample
    // sample bilinear
}

pub trait Draw {
    type Pixel: Clone + Default;

    // Define a method called set which takes a mutable reference to self, a position, and a value
    fn set(&mut self, p: IVec2, value: Self::Pixel);

    // draw rect lo hi etc

    fn draw_circle(&mut self, c: IVec2, r: i32, fill: Self::Pixel) {
        for i in c.x - r..=c.x + r {
            for j in c.y - r..=c.y + r {
                let p = ivec2(i,j);
                let u = p - c;
                if u.dot(&u) <= r*r {
                    self.set(p, fill.clone());
                }
            }
        }
    }
}

impl<T: Clone + Default> Draw for Texture<T> {
    type Pixel = T;

    fn set(&mut self, p: IVec2, value: T) {
        if p.x < 0 || p.x >= self.wh.x || p.y < 0 || p.y >= self.wh.y {
            return;
        }
        let bucket = self.data.get_mut((p.x + p.y * self.wh.x) as usize);
        if bucket.is_none() {
            return;
        }
        *bucket.unwrap() = value;
    }
}

#[test]
fn test_texture() {
    let mut t: Texture<bool> = Texture::new(ivec2(4,4), false);
    t.set(ivec2(0,0), true);
    assert_eq!(t.sample(ivec2(0, 0)), true);
    assert_eq!(t.sample(ivec2(1, 0)), false);
    assert_eq!(t.sample(ivec2(1, 1)), false);
    
    t.draw_circle(ivec2(0,0), 8, true);
    assert_eq!(t.sample(ivec2(0, 0)), true);
    assert_eq!(t.sample(ivec2(3, 3)), true);
    
    t.fill(false);
    assert_eq!(t.sample(ivec2(0, 0)), false);
    assert_eq!(t.sample(ivec2(3, 3)), false);
}

// lol le map on texture etc. or transform whatever &mut, f<Pixel->Pixel>

#[test]
fn test_save() {
    let wh = ivec2(256,256);
    let mut t: Texture<bool> = Texture::new(wh, false);
    t.draw_circle(ivec2(64,64), 32, true);
    t.save(std::path::Path::new("./test.png"), |b| if b { [255, 255, 255, 255]} else { [0, 0, 0, 255]})
}