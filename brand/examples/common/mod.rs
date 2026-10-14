use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::{fs, io};

macro_rules! run_all_rngs {
    ($test:ident, $count:expr) => {{
        use brand::lcg::{Lcg, TinyLcg};
        use brand::xor::{Xorshift64, Xorshift64Star, Xoshiro256SS, XoroshiroPlus};
        use brand::splitmix::{Splitmix64};
        use brand::pcg::{Pcg32};

        let mut rng = Lcg::new(1902302);
        $test(&mut rng, $count)?;

        let mut rng = TinyLcg::new(1902302);
        $test(&mut rng, $count)?;

        let mut rng = Xorshift64::new(1902302);
        $test(&mut rng, $count)?;

        let mut rng = Xorshift64Star::new(1902302);
        $test(&mut rng, $count)?;

        let mut rng = Xoshiro256SS::new(1902302);
        $test(&mut rng, $count)?;

        let mut rng = XoroshiroPlus::new(1902302);
        $test(&mut rng, $count)?;

        let mut rng = Splitmix64::new(1902302);
        $test(&mut rng, $count)?;

        let mut rng = Pcg32::new(1902302);
        $test(&mut rng, $count)?;
    }};
}
pub(crate) use run_all_rngs;

#[allow(dead_code)]
pub fn write_p6(filename: &str, data: &[u8], width: usize, height: usize) -> io::Result<()> {
    let mut writer = BufWriter::new(open_file(&filename)?);
    
    write!(writer, "P6\n{width} {height}\n255\n")?;
    for value in data {
        let value = *value;
        writer.write_all(&[value, value, value])?;
    }
    writer.flush()?;
    println!("Created {}", &filename);
    
    Ok(())
}

#[allow(dead_code)]
fn open_file(filename: &str) -> io::Result<File> {
    let path = Path::new(&filename);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)?;
    
    Ok(file)
}