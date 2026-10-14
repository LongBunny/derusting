mod common;

use crate::common::{run_all_rngs, write_p6};
use brand::Rng;
use std::any::type_name;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

fn main() -> std::io::Result<()> {
    run_all_rngs!(noise, 0);

    Ok(())
}

fn noise<R: Rng>(rng: &mut R, _iterations: usize) -> std::io::Result<()> {
    let name = type_name::<R>().rsplit("::").next().unwrap().to_uppercase();
    println!("=========== {} ===========", name);

    let mut low = Vec::with_capacity(WIDTH * HEIGHT);
    let mut high = Vec::with_capacity(WIDTH * HEIGHT);
    for _ in 0..WIDTH * HEIGHT {
        let value = rng.next_u64();
        low.push(value as u8);
        high.push((value >> 56) as u8);
    }
    
    write_p6(&format!("out/{name}/noise_lo.ppm"), &low, WIDTH, HEIGHT)?;
    write_p6(&format!("out/{name}/noise_hi.ppm"), &high, WIDTH, HEIGHT)?;

    println!("\n");

    Ok(())
}
