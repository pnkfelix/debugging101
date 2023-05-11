use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let output_filename = std::env::args().nth(1).expect("Need input filename as arg 1");
    let mut output_file = File::create(output_filename)?;
    for elem in DATA {
        write!(output_file, "{}", elem)?;
        write!(output_file, "\n")?;
    }
    Ok(())
}

static DATA: &[u64; 10] =
    &[
        10,
        20,
        30,
        40,
        50,
        15,
        60,
        70,
        80,
        90,
    ];
