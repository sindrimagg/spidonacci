use crate::time;
use std::fs::File;
use std::io::{BufWriter, Write, Result};
use num_bigint::BigUint;

pub fn write_file(name: &str, f: fn(usize) -> BigUint) -> Result<()> {
    let file = File::create(format!("out/{}.dat", name))?;
    let mut writer = BufWriter::new(file);

    let mut i = 0;
    loop {
        let res = time::time_fn(i, f);
        write_line(&mut writer, format!("{:11}, {:11}", i, res))?;

        i += 10_000;
        if res > 1_100_000_000 {
            break;
        }
    }

    writer.flush()?;
    Ok(())
}

fn write_line(writer: &mut BufWriter<File>, line: String) -> Result<()> {
    writeln!(writer, "{}", line)
}

