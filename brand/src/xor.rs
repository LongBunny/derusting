use crate::Rng;

pub struct Xorshift64 {
    state: u64,
}

impl Rng for Xorshift64 {
    fn new(seed: u64) -> Self {
        Self {
            state: seed,
        }
    }

    fn next_u64(&mut self) -> u64 {
        let mut value = self.state;
        value ^= value << 13;
        value ^= value >> 7;
        value ^= value << 17;
        self.state = value;
        value
    }
}