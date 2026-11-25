use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::process::ExitCode;

fn run() -> Result<(), String> {
    let mut args = env::args_os().skip(1);
    let path = args.next();
    if args.next().is_some() {
        return Err("usage: jsonfmt [FILE]".into());
    }

    // Parse the entire input before writing anything to stdout.
    let parsed = match path {
        Some(path) => {
            let file = File::open(&path).map_err(|err| format!("cannot open {:?}: {err}", path))?;
            serde_json::from_reader::<_, Value>(BufReader::new(file))
        }
        None => serde_json::from_reader::<_, Value>(io::stdin().lock()),
    };
    let value = parsed.map_err(|err| {
        if err.is_io() {
            format!("cannot read input: {err}")
        } else {
            format!("invalid JSON: {err}")
        }
    })?;

    // serde_json's pretty serializer uses two spaces per indentation level.
    let mut stdout = io::stdout().lock();
    serde_json::to_writer_pretty(&mut stdout, &value)
        .map_err(|err| format!("cannot write output: {err}"))?;
    writeln!(stdout).map_err(|err| format!("cannot write output: {err}"))?;
    stdout
        .flush()
        .map_err(|err| format!("cannot flush output: {err}"))
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("jsonfmt: {err}");
            ExitCode::FAILURE
        }
    }
}
