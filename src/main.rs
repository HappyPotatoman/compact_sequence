use structopt::StructOpt;

use compact_sequence::mode::Mode;

use compact_sequence::processors::file_processor::{FileProcessor, TextProcessor};

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

    let processor: Box<dyn FileProcessor> = match input_path {
        path if path.ends_with(".txt") => Box::new(TextProcessor),
        _ => return Err("Unsupported file format or invalid path".into()),
    };

    if opt.unpack {
        processor.unpack(input_path, &output_path, mode)?;
    } else {
        processor.compress(input_path, &output_path, mode)?;
    }
    // } else if input_path.ends_with('/') || input_path.ends_with('\\') {
    //     if opt.unpack {
    //         processors::directory_processing::unpack_directory(input_path, &output_path, &mode)?;
    //     } else {
    //         processors::directory_processing::compress_directory(input_path, &output_path, &mode)?;
    //     }
    // } else {
    //     println!("Error: Unsupported file format or invalid path.");
    //     return Err("Unsupported file format or invalid path".into());
    // }

    println!("File processing completed!");
    Ok(())
}