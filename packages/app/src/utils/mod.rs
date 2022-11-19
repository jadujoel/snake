use rand::Rng;
pub fn random(min: usize, max: usize) -> usize {
    rand::thread_rng().gen_range(min..max)
}
