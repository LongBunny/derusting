mod common;

use crate::common::write_p6;
use std::any::type_name;
use brand::lcg::{Lcg, TinyLcg};
use brand::Rng;

fn main() -> std::io::Result<()> {
    let width = 1024;
    let height = 1024;

    let mut rng = Lcg::new(16231235);
    points(&mut rng, width, height)?;
    
    let mut rng = TinyLcg::new(16231235);
    points(&mut rng, width, height)?;

    Ok(())
}

fn points<R: Rng>(rng: &mut R, width: usize, height: usize) -> std::io::Result<()> {
    let name = type_name::<R>().rsplit("::").next().unwrap().to_uppercase();
    println!("=========== {} ===========", name);

    let mut data = vec![0u8; width * height];
    
    for _ in 0..100_000 {
        let x = rng.next_f64();
        let y = rng.next_f64();
        
        let px = (x * width as f64) as usize;
        let py = (y * height as f64) as usize;
        
        let idx = py * width + px;
        data[idx] = 255;
    }
    
    write_p6(&format!("out/{name}/points_u8.ppm"), &data, width, height)?;

    Ok(())
}
