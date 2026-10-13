use std::any::type_name;

use brand::{Rng, Lcg};

fn main() {
    let mut rng = Lcg::new(16231235);
    frequencies(&mut rng, 1_000_000);
}

fn frequencies<R: Rng>(rng: &mut R, samples: usize) {
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
}
