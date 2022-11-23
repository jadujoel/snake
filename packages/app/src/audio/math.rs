pub fn linear_from_decibel(decibel: f32) -> f32 {
    10.0_f32.powf(decibel / 20.0)
}

#[allow(dead_code, unused_variables, unused_assignments)]
pub fn decibel_from_linear(linear: f32) -> f32 {
    20.0 * linear.log10()
}
