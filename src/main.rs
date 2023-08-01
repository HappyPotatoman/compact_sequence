use std::env;

use compact_sequence::{
    compress_to_file,
    unpack_from_file,
}; 

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: cargo run <input_file_name> -u");
        return Ok(());
    }

    let input_file_name = &args[1];

    if args.len() >= 3 && args[2] == "-u" {
        unpack_from_file(input_file_name)?;
    } else {
        compress_to_file(input_file_name)?;
    }

    println!("File processing completed!");
    Ok(())
}