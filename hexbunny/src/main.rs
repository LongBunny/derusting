use std::env;
use std::fmt::Write;
use std::fs;

const WIDTH: usize = 16;

fn main() {
    let mut args = env::args();
    let program = args
        .next()
        .expect("The program name should always be the first arg");
    let Some(path) = args.next() else {
        eprintln!("Usage: {program} <input>");
        return;
    };
    let bytes: Vec<u8> = fs::read(path).unwrap();
    print!("{}", hex_dump(&bytes).unwrap());
}

fn hex_dump(values: &[u8]) -> Result<String, std::fmt::Error> {
    let mut s = String::new();
    let (chunks, remainder) = values.as_chunks::<WIDTH>();
    for (row, chunk) in chunks.iter().enumerate() {
        format_chunk(&mut s, chunk, row)?;
    }
    if !remainder.is_empty() {
        format_chunk(&mut s, remainder, chunks.len())?;
    }
    Ok(s)
}

fn format_chunk(s: &mut String, chunk: &[u8], row: usize) -> std::fmt::Result {
    debug_assert!(chunk.len() <= WIDTH);
    write!(s, "{:08X} ", row * WIDTH)?;
    for (i, v) in chunk.iter().enumerate() {
        write!(s, "{v:02X} ")?;
        if i == (WIDTH / 2) - 1 {
            write!(s, " ")?;
        }
    }
    if chunk.len() < WIDTH / 2 {
        write!(s, " ")?;
    }
    fill_gap(s, chunk.len(), 3);
    write!(s, " |")?;
    for &v in chunk {
        let c = char::from(v);
        if c.is_ascii() && !c.is_ascii_control() {
            write!(s, "{}", c)?;
        } else {
            s.push('.');
        }
    }
    fill_gap(s, chunk.len(), 1);
    writeln!(s, "|")?;
    Ok(())
}

fn fill_gap(s: &mut String, len: usize, spacing: usize) {
    for _ in 0..WIDTH - len {
        for _ in 0..spacing {
            s.push(' ');
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        assert_eq!(hex_dump(b"").unwrap(), "");
    }
    
    #[test]
    fn less_than_len_16() {
        assert_eq!(
            hex_dump(b"12345").unwrap(),
            "00000000 31 32 33 34 35 |12345 |\n"
        );
    }
    
    #[test]
    fn exactly_len_8() {
        assert_eq!(
            hex_dump(b"12345678").unwrap(),
            "00000000 31 32 33 34 35 36 37 38 |12345678 |\n"
        );
    }
    
    #[test]
    fn exactly_len_9() {
        assert_eq!(
            hex_dump(b"123456789").unwrap(),
            "00000000 31 32 33 34 35 36 37 38 39 |123456789 |\n"
        );
    }
    
    #[test]
    fn exactly_len_16() {
        assert_eq!(
            hex_dump(b"1234567890ABCDEF").unwrap(),
            "00000000 31 32 33 34 35 36 37 38 39 30 41 42 43 44 45 46 |1234567890ABCDEF|\n"
        );
    }
    
    #[test]
    fn two_rows() {
        assert_eq!(
            hex_dump(b"1234567890ABCDEFGHIJKLM").unwrap(),
            concat!(
                "00000000 31 32 33 34 35 36 37 38 39 30 41 42 43 44 45 46 |1234567890ABCDEF|\n",
                "00000010 47 48 49 4A 4B 4C 4D |GHIJKLM |\n"
            )
        );
    }
}
