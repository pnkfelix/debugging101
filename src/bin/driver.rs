use std::io::Write;
use std::io::Error as IoError;
use std::process::Command;
use std::string::FromUtf8Error;


#[derive(Debug)]
enum DriverError {
    Io(IoError),
    Utf8(FromUtf8Error),
}

impl From<IoError> for DriverError {
    fn from(e: IoError) -> Self { DriverError::Io(e) }
}
impl From<FromUtf8Error> for DriverError {
    fn from(e: FromUtf8Error) -> Self { DriverError::Utf8(e) }
}

fn main() -> Result<(), DriverError> {
    // dbg!(std::env::vars().collect::<Vec<(String, String)>>());
    let mut args = std::env::args().skip(1);
    let filename = args.next().expect("Need temporary filename as arg 1");

    {
        let mut generate_file = Command::new("cargo");
        let gen_args = ["run", "--bin", "write_data", "--", &filename];
        for arg in gen_args {
            generate_file.arg(arg);
        }
        let gen_output = generate_file.output()?;
        // dbg!(gen_output);
        std::io::stdout().write(&gen_output.stdout)?;
    }

    {
        let mut lookup_in_file = Command::new("cargo");
        let lookup_args = ["run", "--bin", "lookup_index", "--", &filename];
        for arg in lookup_args {
            lookup_in_file.arg(arg);
        }
        for arg in args {
            lookup_in_file.arg(arg);
        }
        let lookup_output = lookup_in_file.output()?;
        // dbg!(lookup_output);
        std::io::stdout().write(&lookup_output.stdout)?;
    }

    Ok(())
}
