use std::{
    env,
    fs::File,
    io::{BufReader, Write},
    process,
};

use compile::compile;

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        process::exit(1);
    } else {
        eprintln!("compiled successfully!")
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);

    let input_path = args.next().ok_or("missing input file")?;
    let output_path = args.next().ok_or("missing output file")?;

    if args.next().is_some() {
        return Err("too many arguments".into());
    }

    let input = File::open(&input_path)?;
    let reader = BufReader::new(input);

    let binary = compile(reader).map_err(|(n, e)| match n {
        Some(n) => format!("compile failed on line {n}: {e:?}"),
        None => format!("compile failed: {e:?}"),
    })?;

    let mut output = File::create(&output_path)?;
    output.write_all(&binary)?;

    Ok(())
}
