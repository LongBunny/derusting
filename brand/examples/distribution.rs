mod common;

use crate::common::run_all_rngs;
use brand::Rng;
use std::any::type_name;

fn main() -> std::io::Result<()> {
    run_all_rngs!(distribution, 10_000_000);

    Ok(())
}

fn distribution<R: Rng>(rng: &mut R, samples: usize) -> std::io::Result<()> {
    println!("=========== {} ===========", type_name::<R>().rsplit("::").next().unwrap().to_uppercase());
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut average = 0f64;
    let mut histogram = [0u64; 10];
    for i in 0..samples {
        let value = rng.next_u64() >> 11;
        let value = value as f64 / (1u64 << 53) as f64;
        if value < min {min = value}
        if value > max {max = value}
        average += (value - average) / (i + 1) as f64;

        let bucket = ((value * 10.0) as usize).min(9);
        histogram[bucket] += 1;
    }

    println!("Stats:");
    println!("min: {}  max: {}  average: {}", min, max, average);
    println!("Distribution: (Each bucket should be around {})", samples / 10);
    for bucket in histogram {
        print!("{}, ", bucket);
    }
    println!("\n");

    Ok(())
}
