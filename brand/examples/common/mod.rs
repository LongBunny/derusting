use std::fs::{File, OpenOptions};
use std::path::Path;
use std::{fs, io};
use std::io::{Write, BufWriter};

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