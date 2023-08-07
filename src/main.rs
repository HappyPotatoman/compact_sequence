use std::str::FromStr;
use structopt::StructOpt;

use compact_sequence::{
    compress_to_file,
    unpack_from_file,
};

mod processors;

#[derive(Debug, StructOpt)]
pub enum Mode {
    RNA,
    DNA,
}

impl FromStr for Mode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rna" => Ok(Mode::RNA),
            "dna" => Ok(Mode::DNA),
            _ => Err("Invalid mode"),
        }
    }
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long)]
    input: String,
    #[structopt(short, long)]
    unpack: bool,
    #[structopt(short, long, default_value = "dna", possible_values = &["rna", "dna"])]
    mode: Mode,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let input_path = &opt.input;
    let mode= &opt.mode;
    println!("Running in {:?} mode", mode);

    if input_path.ends_with(".txt") {
        let output_path = format!("out_{}", input_path);
        if opt.unpack {
            unpack_from_file(input_path, &output_path)?;
        } else {
            compress_to_file(input_path, &output_path)?;
        }
    } else if input_path.ends_with(".fasta") {
        // Add support for fasta processing here if required
    } else if input_path.ends_with('/') || input_path.ends_with('\\') {
        processors::file_processing::process_directory(input_path)?;
    } else {
        println!("Error: Unsupported file format or invalid path.");
        return Err("Unsupported file format or invalid path".into());
    }

    println!("File processing completed!");
    Ok(())
}