use structopt::StructOpt;

use compact_sequence::mode::Mode;

use compact_sequence::processors::processor::{
    Processor,
    TextProcessor, 
    DirectoryProcessor,
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

    let processor: Box<dyn Processor> = match input_path {
        path if path.ends_with(".txt") => Box::new(TextProcessor),
        path if path.ends_with('/') || path.ends_with('\\') => Box::new(DirectoryProcessor::new(vec!["txt".to_string()])),
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