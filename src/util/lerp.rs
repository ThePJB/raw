pub trait Lerp {
    fn lerp(self, other: Self, t: f32) -> Self;
}

impl Lerp for f64 {
    fn lerp(self, other: Self, t: f32) -> Self {
        self * (1.0 - t as Self) + other * t as Self
    }
}

impl Lerp for f32 {
    fn lerp(self, other: Self, t: f32) -> Self {
        self * (1.0 - t) + other * t
    }
}