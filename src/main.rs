use structopt::StructOpt;

use compact_sequence::{
    compress_to_file,
    unpack_from_file,
};
use compact_sequence::mode::Mode;

mod processors;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long)]
    input: String,
    #[structopt(short, long)]
    output: String,
    #[structopt(short, long)]
    unpack: bool,
    #[structopt(short, long, default_value = "dna", possible_values = &["rna", "dna"])]
    mode: Mode,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let input_path = &opt.input;
    let output_path = &opt.output;
    let mode = &opt.mode;
    println!("Running in {:?} mode", mode);

    if input_path.ends_with(".txt") {
        let output_path = format!("out_{}", input_path);
        if opt.unpack {
            unpack_from_file(input_path, &output_path, mode)?;
        } else {
            compress_to_file(input_path, &output_path, mode)?;
        }
    } else if input_path.ends_with(".fasta") {
        // Add support for fasta processing here if required
    } else if input_path.ends_with('/') || input_path.ends_with('\\') {
        if opt.unpack {
            processors::directory_processing::unpack_directory(input_path, &output_path, &mode)?;
        } else {
            processors::directory_processing::compress_directory(input_path, &output_path, &mode)?;
        }
    } else {
        println!("Error: Unsupported file format or invalid path.");
        return Err("Unsupported file format or invalid path".into());
    }

    println!("File processing completed!");
    Ok(())
}