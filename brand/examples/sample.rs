mod common;

use crate::common::run_all_rngs;
use brand::Rng;
use std::any::type_name;

fn main() -> std::io::Result<()> {
    run_all_rngs!(sample, 10);

    Ok(())
}

fn sample<R: Rng>(rng: &mut R, count: usize) -> std::io::Result<()> {
    println!("=========== {} ===========", type_name::<R>().rsplit("::").next().unwrap().to_uppercase());
    for _ in 0..count {
        let value = rng.next_u64();
        println!("{value:20}  {value:016X}");
    }

    Ok(())
}