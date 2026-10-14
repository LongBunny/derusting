mod common;

use crate::common::{run_all_rngs, write_p6};
use brand::Rng;
use std::any::type_name;

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;

fn main() -> std::io::Result<()> {
    run_all_rngs!(points, 0);

    Ok(())
}

fn points<R: Rng>(rng: &mut R, _unused: usize) -> std::io::Result<()> {
    let name = type_name::<R>().rsplit("::").next().unwrap().to_uppercase();
    println!("=========== {} ===========", name);

    let mut data = vec![0u8; WIDTH * HEIGHT];
    
    for _ in 0..100_000 {
        let x = rng.next_f64();
        let y = rng.next_f64();
        
        let px = (x * WIDTH as f64) as usize;
        let py = (y * HEIGHT as f64) as usize;
        
        let idx = py * WIDTH + px;
        data[idx] = 255;
    }
    
    write_p6(&format!("out/{name}/points_u8.ppm"), &data, WIDTH, HEIGHT)?;

    Ok(())
}
