use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use compact_sequence::compress_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the input file
    let input_file = File::open("input.txt")?;
    let reader = BufReader::new(input_file);

    // Open the output file
    let mut output_file = File::create("output.txt")?;

    // Process the data and write to the output file
    for line in reader.lines() {
        let line = line?;
        let compressed_line = compress_string(&line);
        writeln!(output_file, "{}", compressed_line)?;
    }

    println!("File processing completed!");
    Ok(())
}

