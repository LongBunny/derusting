use std::any::type_name;
use std::fs::File;
use std::io::{BufWriter, Write};
use brand::{Lcg, Rng};

fn main() -> std::io::Result<()> {
    let width = 512;
    let height = 512;

    let mut rng = Lcg::new(16231235);
    points(&mut rng, width, height)?;

    Ok(())
}

fn points<R: Rng>(rng: &mut R, width: u32, height: u32) -> std::io::Result<()> {
    let name = type_name::<R>().rsplit("::").next().unwrap().to_uppercase();
    println!("=========== {} ===========", name);

    let filename = String::from(format!("{name}_lo.ppm"));
    let file = File::create(&filename)?;
    let mut writer = BufWriter::new(file);

    write!(writer, "P6\n{width} {height}\n255\n")?;
    for _ in 0..width * height {
        let value = rng.next_u64() as u8;
        writer.write_all(&[value, value, value])?;
    }
    writer.flush()?;
    println!("Created {}", &filename);

    let filename = String::from(format!("{name}_hi.ppm"));
    let file = File::create(&filename)?;
    let mut writer = BufWriter::new(file);

    write!(writer, "P6\n{width} {height}\n255\n")?;
    for _ in 0..width * height {
        let value = (rng.next_u64() >> 56) as u8;
        writer.write_all(&[value, value, value])?;
    }
    writer.flush()?;
    println!("Created {}", &filename);

    Ok(())
}