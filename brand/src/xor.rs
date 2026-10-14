use std::num::NonZeroU64;
use crate::{nonzero_seed, Rng};
use crate::splitmix::Splitmix64;

pub struct Xorshift64 {
    state: NonZeroU64,
}

impl Rng for Xorshift64 {
    fn new(seed: u64) -> Self {
        Self {
            state: nonzero_seed(seed),
        }
    }

    fn next_u64(&mut self) -> u64 {
        let mut result = self.state.get();
        result ^= result << 13;
        result ^= result >> 7;
        result ^= result << 17;
        self.state = NonZeroU64::new(result).expect("Xorshift state transition preserves nonzero state");
        result
    }
}

pub struct Xorshift64Star {
    state: NonZeroU64,
}

impl Rng for Xorshift64Star {
    fn new(seed: u64) -> Self {
        Self {
            state: nonzero_seed(seed),
        }
    }

    fn next_u64(&mut self) -> u64 {
        let mut result = self.state.get();
        result ^= result >> 12;
        result ^= result << 25;
        result ^= result >> 27;
        self.state = NonZeroU64::new(result).expect("Xorshift state transition preserves nonzero state");
        result.wrapping_mul(0x2545F4914F6CDD1D)
    }
}

pub struct Xoshiro256SS {
    state: [u64; 4],
}

impl Rng for Xoshiro256SS {
    fn new(seed: u64) -> Self {
        let mut seed_rng = Splitmix64::new(seed);
        let result = Self {
            state: [
                seed_rng.next_u64(),
                seed_rng.next_u64(),
                seed_rng.next_u64(),
                seed_rng.next_u64()
            ],
        };

        debug_assert_ne!(
            result.state,
            [0; 4],
            "Xoshiro256** state must not be all zero"
        );

        result
    }

    fn next_u64(&mut self) -> u64 {
        let result = self.state[1]
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

        result
    }
}

pub struct XoroshiroPlus {
    state: [u64; 2],
}

impl Rng for XoroshiroPlus {
    fn new(seed: u64) -> Self {
        let mut seed_rng = Splitmix64::new(seed);
        let result = Self {
            state: [
                seed_rng.next_u64(),
                seed_rng.next_u64(),
            ],
        };

        debug_assert_ne!(
            result.state,
            [0; 2],
            "Xoroshiro128+ state must not be all zero"
        );

        result
    }

    fn next_u64(&mut self) -> u64 {
        let s0 = self.state[0];
        let mut s1 = self.state[1];

        let result = s0.wrapping_add(s1);

        s1 ^= s0;
        self.state[0] = s0.rotate_left(24) ^ s1 ^ (s1 << 16);
        self.state[1] = s1.rotate_left(37);

        result
    }
}