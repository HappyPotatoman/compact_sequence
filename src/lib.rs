use std::fs::File;
use std::io::Write;
use std::io::BufReader;
use std::io::BufRead;

pub mod encoders;
pub mod errors;
pub mod processors;

use encoders::nucleotide_to_ascii::{ENCODING_MAP, DECODING_MAP};
use errors::CompressionError;

pub fn compress_string(input: &str) -> Result<String, CompressionError> {
    let input = input.to_uppercase();
    let mut compressed = String::new();

    for chunk in input.as_bytes().chunks(3) {
        let key = String::from_utf8(chunk.to_vec()).unwrap();
        if let Some(encoded_value) = ENCODING_MAP.get(&key) {
            compressed.push_str(&encoded_value.to_string());
        } else {
            return Err(CompressionError::UnknownSequence(key));
        }
    }

    Ok(compressed)
}

pub fn unpack_string(input: &str) -> Result<String, CompressionError> {
    let mut unpacked = String::new();
    let mut previous_was_exclamation = false;

    for ch in input.chars() {
        if ch == '!' {
            previous_was_exclamation = true;
            continue;
        }

        let key = if previous_was_exclamation {
            previous_was_exclamation = false;
            format!("!{}", ch)
        } else {
            ch.to_string()
        };

        if let Some(decoded_value) = DECODING_MAP.get(&key) {
            unpacked.push_str(decoded_value);
        } else {
            return Err(CompressionError::UnknownCharacter(ch));
        }
    }

    Ok(unpacked)
}


pub fn compress_to_file(input: &str, output_file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input_file = File::open(input)?;
    let reader = BufReader::new(input_file);

    let mut output_file = File::create(output_file_name)?;

    for line in reader.lines() {
        let line = line?;
        let compressed_line = compress_string(&line)?;
        writeln!(output_file, "{}", compressed_line)?;
    }

    Ok(())
}

pub fn unpack_from_file(input: &str, output_file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input_file = File::open(input)?;
    let reader = BufReader::new(input_file);

    let mut output_file = File::create(output_file_name)?;

    for line in reader.lines() {
        let line = line?;
        let unpacked_line = unpack_string(&line)?;
        writeln!(output_file, "{}", unpacked_line)?;
    }

    Ok(())
}
