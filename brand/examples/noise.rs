mod common;

use crate::common::write_p6;
use brand::{Rng};
use std::any::type_name;
use brand::lcg::{Lcg, TinyLcg};

fn main() -> std::io::Result<()> {
    let width = 512;
    let height = 512;

    let mut rng = Lcg::new(16231235);
    noise(&mut rng, width, height)?;
    
    let mut rng = TinyLcg::new(16231235);
    noise(&mut rng, width, height)?;

    Ok(())
}

fn noise<R: Rng>(rng: &mut R, width: usize, height: usize) -> std::io::Result<()> {
    let name = type_name::<R>().rsplit("::").next().unwrap().to_uppercase();
    println!("=========== {} ===========", name);

    let mut low = Vec::with_capacity(width * height);
    let mut high = Vec::with_capacity(width * height);
    for _ in 0..width * height {
        let value = rng.next_u64();
        low.push(value as u8);
        high.push((value >> 56) as u8);
    }
    
    write_p6(&format!("out/{name}/noise_lo.ppm"), &low, width, height)?;
    write_p6(&format!("out/{name}/noise_hi.ppm"), &high, width, height)?;

    Ok(())
}
