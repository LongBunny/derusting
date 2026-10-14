use crate::Rng;
use crate::splitmix::Splitmix64;

pub struct Xorshift64 {
    state: u64,
}

impl Rng for Xorshift64 {
    fn new(seed: u64) -> Self {
        let mut seed_rng = Splitmix64::new(seed);
        Self {
            state: seed_rng.next_u64(),
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

pub struct Xorshift64Star {
    state: u64,
}

impl Rng for Xorshift64Star {
    fn new(seed: u64) -> Self {
        let mut seed_rng = Splitmix64::new(seed);
        Self {
            state: seed_rng.next_u64(),
        }
    }

    fn next_u64(&mut self) -> u64 {
        let mut value = self.state;
        value ^= value >> 12;
        value ^= value << 25;
        value ^= value >> 27;
        self.state = value;
        value * 0x2545F4914F6CDD1D
    }
}

pub struct Xoshiro256SS {
    state: [u64; 4],
}

impl Rng for Xoshiro256SS {
    fn new(seed: u64) -> Self {
        let mut seed_rng = Splitmix64::new(seed);
        Self {
            state: [
                seed_rng.next_u64(),
                seed_rng.next_u64(),
                seed_rng.next_u64(),
                seed_rng.next_u64()
            ],
        }
    }

    fn next_u64(&mut self) -> u64 {
        let value = self.state[1]
            .wrapping_mul(5)
            .rotate_left(7)
            .wrapping_mul(9);
        let t = self.state[1] << 17;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;
        self.state[3] = self.state[3].rotate_left(45);

        value
    }
}