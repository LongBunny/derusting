use std::any::type_name;
use std::fs::File;
use std::io::{BufWriter, Write};
use brand::{Lcg, Rng};

fn main() -> std::io::Result<()> {
    let width = 512;
    let height = 512;

    let mut rng = Lcg::new(16231235);
    noise(&mut rng, width, height)?;

    Ok(())
}

fn noise<R: Rng>(rng: &mut R, width: u32, height: u32) -> std::io::Result<()> {
    let name = type_name::<R>().rsplit("::").next().unwrap().to_uppercase();
    println!("=========== {} ===========", name);
    let file = File::create(format!("{name}.ppm"))?;
    let mut writer = BufWriter::new(file);

    write!(writer, "P6\n{width} {height}\n255\n")?;

    for _ in 0..width * height {
        let value = rng.next_u64() as u8;
        writer.write_all(&[value, value, value])?;
    }

    writer.flush()
}