use crate::Rng;

pub struct Pcg32 {
    state: u64,
}

impl Pcg32 {
    const M: u64 = 6364136223846793005;
    const I: u64 = 1442695040888963407;
}

impl Rng for Pcg32 {
    fn new(seed: u64) -> Self {
        let mut rng = Self { state: 0 };

        rng.next_u64();
        rng.state = rng.state.wrapping_add(seed);
        rng.next_u64();

        rng
    }

    fn next_u64(&mut self) -> u64 {
        let hi = self.next_u32() as u64;
        let lo = self.next_u32() as u64;

        (hi << 32) | lo
    }

    fn next_u32(&mut self) -> u32 {
        let x = self.state;
        let count = x >> 59;

        self.state = x.wrapping_mul(Self::M).wrapping_add(Self::I);

        let xorshifted = (((x >> 18) ^ x) >> 27) as u32;
        xorshifted.rotate_right(count as u32)
    }
}