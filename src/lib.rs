use rayon::prelude::*;
use std::fs::File;
use std::io::{BufReader, BufRead, Write, BufWriter};

pub mod encoders;
pub mod errors;
pub mod processors;

use encoders::nucleotide_to_ascii::{ENCODING_MAP, DECODING_MAP};
use errors::CompressionError;

fn compress_string(input: &str) -> Result<String, CompressionError> {
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

fn unpack_string(input: &str) -> Result<String, CompressionError> {
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

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let compressed_lines: Vec<_> = lines.par_iter()
        .map(|line| compress_string(line))
        .collect::<Result<_, _>>()?;

    let mut output_file = BufWriter::new(File::create(output_file_name)?);

    for compressed_line in compressed_lines {
        writeln!(output_file, "{}", compressed_line)?;
    }

    Ok(())
}

pub fn unpack_from_file(input: &str, output_file_name: &str) -> Result<(), Box<dyn std::error::Error>> {

    let input_file = File::open(input)?;
    let reader: Vec<_> = BufReader::new(input_file).lines().collect();
    let output_file = File::create(output_file_name)?;
    let mut writer = BufWriter::new(output_file);

    reader.par_iter()
          .map(|line| {
              let line = line.as_ref().unwrap();
              unpack_string(&line)
          })
          .collect::<Result<Vec<_>, _>>()?
          .into_iter()
          .try_for_each(|unpacked_line| writeln!(writer, "{}", unpacked_line))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_string() {
        let test_strings = vec!["AAAA", "AC", "AAAACCCGTT", "AGGGGCCCCTTTTAA", ""];
        for s in test_strings {
            let compressed = compress_string(s).unwrap();
            let expected_len = (s.len() + 2) / 3;
            assert_eq!(compressed.len(), expected_len);
        }
    }

    #[test]
    fn test_unpack_string() {
        let test_strings = vec!["A", "Aq1", "123", "5", ""];
        for s in test_strings {
            let unpacked = unpack_string(s).unwrap();
            assert!(unpacked.len() >= s.len());
        }
    }

    #[test]
    fn test_compress_to_file() {
        let input_strings = vec!["AAAA", "AC", "AAAACCCGTT", "AGGGGCCCCTTTTAA",""];
        let input_file_name = "test_input_compress.txt";
        let mut input_file = File::create(input_file_name).unwrap();
        for s in &input_strings {
            writeln!(input_file, "{}", s).unwrap();
        }

        let output_file_name = "test_output_compress.txt";
        compress_to_file(input_file_name, output_file_name).unwrap();

        let output_file = File::open(output_file_name).unwrap();
        let reader = BufReader::new(output_file);
        let output_lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();

        for (input, output) in input_strings.iter().zip(output_lines.iter()) {
            let expected_len = (input.len() + 2) / 3;
            assert_eq!(output.len(), expected_len);
        }

        std::fs::remove_file(input_file_name).unwrap();
        std::fs::remove_file(output_file_name).unwrap();
    }

    #[test]
    fn test_unpack_from_file() {
        let input_strings = vec!["A4", "AC", "A4C3G1T2", "AG4C4T4A2"];
        let input_file_name = "test_input_unpack.txt";
        let mut input_file = File::create(input_file_name).unwrap();
        for s in &input_strings {
            writeln!(input_file, "{}", s).unwrap();
        }

        let output_file_name = "test_output_unpack.txt";
        unpack_from_file(input_file_name, output_file_name).unwrap();

        let output_file = File::open(output_file_name).unwrap();
        let reader = BufReader::new(output_file);
        let output_lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();

        for (input, output) in input_strings.iter().zip(output_lines.iter()) {
            assert!(output.len() >= input.len());
        }

        std::fs::remove_file(input_file_name).unwrap();
        std::fs::remove_file(output_file_name).unwrap();
    }
}

