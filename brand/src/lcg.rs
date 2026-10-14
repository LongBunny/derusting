use crate::Rng;

pub struct Lcg {
    state: u64,
}

impl Lcg {
    const A: u64 = 6364136223846793005;
    const C: u64 = 9754186451795953191;
}

impl Rng for Lcg {
    fn new(seed: u64) -> Self { Self { state: seed } }
    
    fn next_u64(&mut self) -> u64 {
        self.state = self.state
            .wrapping_mul(Self::A)
            .wrapping_add(Self::C);
        self.state
    }
}

pub struct TinyLcg {
    state: u8,
}

impl TinyLcg {
    const A: u8 = 5;
    const C: u8 = 1;
}

impl Rng for TinyLcg {
    fn new(seed: u64) -> Self {
        Self { state: seed as u8 }
    }

    fn next_u64(&mut self) -> u64 {
        let lo = self.next_u32() as u64;
        let hi = self.next_u32() as u64;

        (hi << 32) | lo
    }

    fn next_u32(&mut self) -> u32 {
        let lo = self.next_u16() as u32;
        let hi = self.next_u16() as u32;

        (hi << 16) | lo
    }

    fn next_u16(&mut self) -> u16 {
        let lo = self.next_u8() as u16;
        let hi = self.next_u8() as u16;

        (hi << 8) | lo
    }

    fn next_u8(&mut self) -> u8 {
        self.state = self.state
            .wrapping_mul(Self::A)
            .wrapping_add(Self::C);

        self.state
    }
}