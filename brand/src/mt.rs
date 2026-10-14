use crate::Rng;

pub struct Mt19937 {
    state: [u32; Self::STATE_COUNT],
    state_index: i32
}

impl Mt19937 {
    const STATE_COUNT: usize = 624;
    const F: u32
}

impl Rng for Mt19937 {
    fn new(seed: u64) -> Self {
        let mut result = Self {
            state: [0; Self::STATE_COUNT],
            state_index: 0
        };

        let mut seed = seed as u32;

        result.state[0] = seed;

        for i in 1..Self::STATE_COUNT {
            seed =
        }

        result
    }

    fn next_u64(&mut self) -> u64 {
        todo!()
    }
}