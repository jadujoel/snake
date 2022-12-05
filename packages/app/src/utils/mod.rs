use rand::Rng;
pub fn random_usize(min: usize, max: usize) -> usize {
    rand::thread_rng().gen_range(min..max)
}

pub fn random_f32(min: f32, max: f32) -> f32 {
    rand::thread_rng().gen_range(min..max)
}

pub fn clamp(v: f32, min: f32, max: f32) -> f32 {
    if v < min {
        return min;
    } else if v > max {
        return max;
    }
    v
}

pub fn f64_from_usize(x: usize) -> f64 {
    let x: u32 = x.try_into().unwrap();
    x.try_into().unwrap()
}

#[allow(dead_code)]
pub fn f32_from_usize(x: usize) -> f32 {
    let x: u16 = x.try_into().unwrap();
    x.try_into().unwrap()
}

pub fn u32_from_usize(x: usize) -> u32 {
    x.try_into().unwrap()
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn u32_from_f64(x: f64) -> u32 {
    x.clamp(0.0, u32::MAX.try_into().unwrap()) as u32
}
