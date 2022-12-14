use std::io::ErrorKind;
use std::path::PathBuf;
use std::fs::read_to_string;
use clap::Parser;

use regex::Regex;

/// Takes a file (values) that contains a list of strings and outputs a list of which strings do not appear in the input file (input)
#[derive(Parser, Debug)]
struct Args {
   /// The path to the file containing the values to search for. (cases separated by newlines)
   #[clap(short, long)]
   values: PathBuf,

   /// The path to the file containing contents to match against
   #[clap(short, long)]
   input: PathBuf,
}

fn open_error(err: ErrorKind, filename: PathBuf) -> ! {
    let msg: String = match err {
        ErrorKind::NotFound =>         format!("File not found: {:?}", filename),
        ErrorKind::PermissionDenied => format!("Permission denied: {:?}", filename),
        ErrorKind::AlreadyExists
        | ErrorKind::InvalidInput =>   unreachable!("Program doesn't create/modify files"),
        _ => format!("Unhandled ErrorKind: {:?}", err),
    };
    panic!("{}", msg);
}

fn main() {
    let args = Args::parse();

    let values_raw: String = match read_to_string(&args.values.as_path()) {
        Ok(s) => s,
        Err(e) => open_error(e.kind(), args.values.clone()),
    };
    let values_lf: String = values_raw.replace("\r", "");
    let values: Vec<&str> = values_lf.split("\n").collect();
    
    let input: String = match read_to_string(&args.input.as_path()) {
        Ok(input) => input,
        Err(e) => open_error(e.kind(), args.input.clone()),
    };

    let mut missing: Vec<&str> = Vec::new();
    for value in values {
        if !input.contains(value) {
            missing.push(value);
        }
    }
    
    println!("{}", missing.join("\n"));
}