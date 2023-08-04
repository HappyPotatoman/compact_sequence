use std::env;
use compact_sequence::{
    compress_to_file,
    unpack_from_file,
};
mod processors;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <input_file_or_directory> [-u]");
        return Ok(());
    }

    let input_path = &args[1];

    if input_path.ends_with(".txt") {
        let output_path = format!("{}_output.txt", input_path);
        if args.len() >= 3 && args[2] == "-u" {
            unpack_from_file(input_path, &output_path)?;
        } else {
            compress_to_file(input_path, &output_path)?;
        }
    } else if input_path.ends_with(".fasta") {
        // Add support for fasta processing here if required - in the future
    } else if input_path.ends_with('/') || input_path.ends_with('\\') {
        processors::file_processing::process_directory(input_path)?;
    } else {
        println!("Error: Unsupported file format or invalid path.");
        return Err("Unsupported file format or invalid path".into());
    }

    println!("File processing completed!");
    Ok(())
}