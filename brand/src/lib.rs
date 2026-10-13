pub trait Rng {
    fn new(seed: u64) -> Self;
    fn next_u64(&mut self) -> u64;
    // pub fn next_f64() -> f64;
}

pub struct Lcg {
    state: u64,
}

impl Lcg {
    const A: u64 = 6364136223846793005;
    const C: u64 = 9754186451795953191;
    const M: u64 = u64::MAX;
    pub const MAX: u64 = Self::M;
}

impl Rng for Lcg {
    fn new(seed: u64) -> Self { Self { state: seed } }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state
            .wrapping_mul(Self::A)
            .wrapping_add(Self::C)
            & Self::M;
        self.state
    }
}