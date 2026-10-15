use crate::Rng;

pub struct Mt19937 {
    state: [u32; Self::STATE_COUNT],
    state_index: usize,
}

impl Mt19937 {
    const STATE_COUNT: usize = 624;
    const M: u32 = 397;
    const F: u32 = 1812433253;
    const W: u32 = 32;
    const R: u32 = 31;
    const UMASK: u32 = 0xffffffff << Self::R;
    const LMASK: u32 = 0xffffffff >> (Self::W - Self::R);
    const A: u32 = 0x9908b0df;
    const U: u32 = 11;
    const S: u32 = 7;
    const B: u32 = 0x9d2c5680;
    const T: u32 = 15;
    const C: u32 = 0xefc60000;
    const L: u32 = 18;
}

impl Rng for Mt19937 {
    fn new(seed: u64) -> Self {
        let mut result = Self {
            state: [0; Self::STATE_COUNT],
            state_index: 0,
        };

        let mut seed = seed as u32;

        result.state[0] = seed;

        for i in 1..Self::STATE_COUNT {
            seed = Self::F
                .wrapping_mul(seed ^ (seed >> (Self::W - 2)))
                .wrapping_add(i as u32);
            result.state[i] = seed;
        }

        result
    }

    fn next_u64(&mut self) -> u64 {
        let hi = self.next_u32() as u64;
        let lo = self.next_u32() as u64;

        (hi << 32) | lo
    }

    fn next_u32(&mut self) -> u32 {
        let n = Self::STATE_COUNT;
        let k = self.state_index;

        let j = (k + 1) % n;

        let x = (self.state[k] & Self::UMASK) | (self.state[j] & Self::LMASK);

        let mut x_a = x >> 1;

        if x & 1 != 0 {
            x_a ^= Self::A;
        }

        let j = (k + Self::M as usize) % n;

        let x = self.state[j] ^ x_a;
        self.state[k] = x;

        self.state_index = (k + 1) % n;

        let mut y = x ^ (x >> Self::U);
        y ^= (y << Self::S) & Self::B;
        y ^= (y << Self::T) & Self::C;
        y ^= y >> Self::L;

        y
    }
}


#[test]
fn test_mt19937_reference() {
    let mut rng = Mt19937::new(5489);
    
    let expected = [
        3499211612,
        581869302,
        3890346734,
        3586334585,
        545404204,
        4161255391,
        3922919429,
        949333985,
        2715962298,
        1323567403,
    ];
    
    for value in expected {
        assert_eq!(rng.next_u32(), value);
    }
}
