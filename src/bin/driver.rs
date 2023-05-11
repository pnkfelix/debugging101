use std::process::Command;

fn main() -> std::io::Result<()> {
    // dbg!(std::env::vars().collect::<Vec<(String, String)>>());
    let mut args = std::env::args().skip(1);
    let filename = args.next().expect("Need temporary filename as arg 1");
    let input_key = args.next().expect("Need key as arg 2");

    {
        let mut generate_file = Command::new("cargo");
        let gen_args = ["run", "--bin", "write_data", "--", &filename];
        for arg in gen_args {
            generate_file.arg(arg);
        }
        let gen_output = generate_file.output()?;
        dbg!(gen_output);
    }

    {
        let mut lookup_in_file = Command::new("cargo");
        let lookup_args = ["run", "--bin", "lookup_index", "--", &filename, &input_key];
        for arg in lookup_args {
            lookup_in_file.arg(arg);
        }
        let lookup_output = lookup_in_file.output()?;
        dbg!(lookup_output);
    }

    Ok(())
}
