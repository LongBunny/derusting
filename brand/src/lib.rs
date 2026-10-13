pub trait Rng {
    fn new(seed: u64) -> Self;
    fn next_u64(&mut self) -> u64;
    // pub fn next_f64() -> f64;
}

pub struct Lcg {
    state: u64,
}

impl Lcg {
    const A: u64 = 48271;
    const C: u64 = 0;
    const M: u64 = 2u64.pow(31) - 1;
    pub const MAX: u64 = Self::M;    
}

impl Rng for Lcg {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state
            .wrapping_mul(Self::A)
            .wrapping_add(Self::C)
            & Self::M;
        self.state
    }
}