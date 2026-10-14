mod common;

use crate::common::run_all_rngs;
use brand::Rng;
use std::any::type_name;

fn main() -> std::io::Result<()> {
    run_all_rngs!(frequencies, 1_000_000);
    
    Ok(())
}

fn frequencies<R: Rng>(rng: &mut R, samples: usize) -> std::io::Result<()> {
    println!("=========== {} ===========", type_name::<R>().rsplit("::").next().unwrap().to_uppercase());
    
    let mut bits = [0u64; 64];
    for _ in 0..samples {
        let value = rng.next_u64();
        for i in 0..64 {
            let bit = (value >> i) & 1 == 1;
            if bit {
                bits[i] += 1;
            }
        }
    }
    
    for (i, bit) in bits.iter().enumerate() {
        let bit = *bit;
        let frequency = 100f64 / samples as f64 * bit as f64;
        println!("Bit {:2}: {:.4}%", i, frequency);
    }
    
    Ok(())
}
