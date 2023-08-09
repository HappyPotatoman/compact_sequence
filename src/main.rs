use structopt::StructOpt;
use std::path::Path;
use compact_sequence::file_extensions::*;

use compact_sequence::mode::Mode;

use compact_sequence::processors::processor::{
    DirectoryProcessor,
    FastaProcessor,
    Processor,
    TextProcessor, 
};

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

    let path = Path::new(input_path);
    let processor: Box<dyn Processor> = match path.extension().and_then(|s| s.to_str()) {
        Some(ext) if is_fasta_extension(ext) => Box::new(FastaProcessor),
        Some(ext) if is_text_extension(ext) => Box::new(TextProcessor),
        _ if path.is_dir() => Box::new(DirectoryProcessor::new(vec!["txt".to_string()])),
        _ => return Err("Unsupported file format or invalid path".into()),
    };

    if opt.unpack {
        processor.unpack(input_path, &output_path, mode)?;
    } else {
        processor.compress(input_path, &output_path, mode)?;
    }

    println!("File processing completed!");
    Ok(())
}