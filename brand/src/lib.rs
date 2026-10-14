use std::num::NonZeroU64;
use crate::splitmix::Splitmix64;

pub mod lcg;
pub mod xor;
pub mod splitmix;
pub mod pcg;

pub trait Rng {
    fn new(seed: u64) -> Self;
    fn next_u64(&mut self) -> u64;
    
    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }
    fn next_u16(&mut self) -> u16 {
        (self.next_u64() >> 48) as u16
    }
    fn next_u8(&mut self) -> u8 {
        (self.next_u64() >> 56) as u8
    }
    
    fn next_f64(&mut self) -> f64 {
        let value = self.next_u64() >> 11;
        value as f64 / (1u64 << 53) as f64
    }
    
    fn next_f32(&mut self) -> f32 {
        self.next_f64() as f32
    }
}

fn nonzero_seed(seed: u64) -> NonZeroU64 {
    let mut seed_rng = Splitmix64::new(seed);

    loop {
        if let Some(state) = NonZeroU64::new(seed_rng.next_u64()) {
            break state;
        }
    }
}

