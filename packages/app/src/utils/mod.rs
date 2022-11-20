use rand::Rng;
pub fn random(min: usize, max: usize) -> usize {
    rand::thread_rng().gen_range(min..max)
}

pub fn clamp(v: f32, min: f32, max: f32) -> f32 {
    if v < min {
        return min;
    }
    else if v > max {
        return max;
    }
    v
}
