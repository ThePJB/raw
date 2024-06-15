use std::time::SystemTime;

pub fn random_seed() -> u32 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map(|x| x.as_nanos() as u32).unwrap_or(1)
}

pub trait KHash {
    fn next(&mut self);
    fn as_f32(&self) -> f32;
    fn chance(&mut self, percent: f32) -> bool {
        self.next();
        self.as_f32() < percent
    }
}
impl KHash for u32 {
    fn next(&mut self) {
        *self = (*self ^ 2747636419).wrapping_mul(2654435769);
        *self = (*self ^ (*self >> 16)).wrapping_mul(2654435769);
        *self = (*self ^ (*self >> 16)).wrapping_mul(2654435769);
    }
    fn as_f32(&self) -> f32 {
        *self as f32 / u32::MAX as f32
    }
}