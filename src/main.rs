#![feature(termination_trait)]

extern crate regex;
extern crate clap;

use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::fs::OpenOptions;
use std::io;

use regex::bytes::Regex;
use clap::{Arg, App};

fn out_file_name(base: &str, ix: usize, extension: Option<&str>) -> String {
    match extension {
        None => format!("{}.{:03}", base, ix),
        Some(ext) => format!("{}.{:03}.{}", base, ix, ext)
    }
}

fn main() -> Result<(), io::Error> {
    let matches = App::new("Betaflight blackbox log splitter")
                        .version("1.0")
                        .author("Ilya Epifanov <elijah.epifanov@gmail.com>")
                        .arg(Arg::with_name("INPUT")
                            .help("Input .bbl file possibly containing multiple logs")
                            .required(true)
                            .index(1))
                        .get_matches();

    let filename = Path::new(matches.value_of("INPUT").unwrap());
    let filename_base = filename.file_stem().unwrap().to_string_lossy();
    let filename_extension_string = filename.extension()
        .map(|e| String::from(e.to_string_lossy()));
    let filename_extension = filename_extension_string
        .as_ref()
        .map(String::as_ref);

    let mut input_file = OpenOptions::new()
        .read(true)
        .open(filename)?;
    let mut bytes = Vec::new();
    input_file.read_to_end(&mut bytes)?;
    drop(input_file);

    let re = Regex::new(r"(?-u)\xffEnd of log\x00\xff*").unwrap();
    for (ix, mut chunk) in re.split(&mut bytes).enumerate() {
        let mut out_file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(filename.with_file_name(out_file_name(&filename_base, ix, filename_extension)))?;
        out_file.write_all(chunk)?;
        out_file.write_all(b"\xffEnd of log\x00")?;
    }

    Ok(())
}
