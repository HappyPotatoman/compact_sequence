use rayon::prelude::*;
use std::fs::File;
use std::io::{BufReader, BufRead, Write, BufWriter};

pub mod encoders;
pub mod errors;
pub mod processors;
pub mod mode;
pub mod file_extensions;


use mode::Mode;
use encoders::Encoder;
use errors::{CompressionError, FastaCompressionError, FastaUnpackingError};

fn compress_string(input: &str, mode: &Mode) -> Result<String, CompressionError> {
    let encoder = Encoder::new(mode);

    let encoding_map = &encoder.encoding_map();

    let input = input.to_uppercase();
    let mut compressed = String::new();

    for chunk in input.as_bytes().chunks(3) {
        let key = String::from_utf8(chunk.to_vec()).unwrap();
        if let Some(encoded_value) = encoding_map.get(&key) {
            compressed.push_str(&encoded_value.to_string());
        } else {
            return Err(CompressionError::UnknownSequence(key));
        }
    }

    Ok(compressed)
}

fn unpack_string(input: &str, mode: &Mode) -> Result<String, CompressionError> {
    let encoder = Encoder::new(mode);

    let decoding_map = &encoder.decoding_map();

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

        if let Some(decoded_value) = decoding_map.get(&key) {
            unpacked.push_str(decoded_value);
        } else {
            return Err(CompressionError::UnknownCharacter(ch));
        }
    }

    Ok(unpacked)
}


pub fn compress_to_file(input: &str, output_file_name: &str, mode: &Mode) -> Result<(), Box<dyn std::error::Error>> {
    let input_file = File::open(input)?;
    let reader = BufReader::new(input_file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let compressed_lines: Vec<_> = lines.par_iter()
        .map(|line| compress_string(line, &mode))
        .collect::<Result<_, _>>()?;

    let mut output_file = BufWriter::new(File::create(output_file_name)?);

    for compressed_line in compressed_lines {
        writeln!(output_file, "{}", compressed_line)?;
    }

    Ok(())
}

pub fn unpack_from_file(input: &str, output_file_name: &str, mode: &Mode) -> Result<(), Box<dyn std::error::Error>> {

    let input_file = File::open(input)?;
    let reader: Vec<_> = BufReader::new(input_file).lines().collect();
    let output_file = File::create(output_file_name)?;
    let mut writer = BufWriter::new(output_file);

    reader.par_iter()
          .map(|line| {
              let line = line.as_ref().unwrap();
              unpack_string(&line, &mode)
          })
          .collect::<Result<Vec<_>, _>>()?
          .into_iter()
          .try_for_each(|unpacked_line| writeln!(writer, "{}", unpacked_line))?;

    Ok(())
}

fn compress_fasta_to_file(input: &str, output_file_name: &str, mode: &Mode) -> Result<(), Box<dyn std::error::Error>> {
    let input_file = File::open(input)?;
    let reader = BufReader::new(input_file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let output_lines: Result<Vec<String>, FastaCompressionError> =
    lines.par_iter().try_fold(
        || Vec::new(),
        |mut acc, line| -> Result<Vec<String>, FastaCompressionError> {
            if line.starts_with('>') {
                acc.push(line.clone());
            } else {
                let compressed_line = compress_string(&line, &mode)?;
                acc.push(compressed_line);
            }
            Ok(acc)
        },
    ).try_reduce(
        || Vec::new(),
        |mut acc, x| -> Result<Vec<String>, FastaCompressionError> {
            acc.extend(x);
            Ok(acc)
        },
    );

    let mut output_file = BufWriter::new(File::create(output_file_name)?);
    output_lines?.iter().for_each(|line| writeln!(output_file, "{}", line).unwrap());

    Ok(())
}

fn unpack_fasta_from_file(input: &str, output_file_name: &str, mode: &Mode) -> Result<(), Box<dyn std::error::Error>> {
    let input_file = File::open(input)?;
    let lines: Vec<String> = BufReader::new(input_file).lines().collect::<Result<_, _>>()?;

    let output_lines: Result<Vec<String>, FastaUnpackingError> =
        lines.par_iter().try_fold(
            || Vec::new(),
            |mut acc, line| -> Result<Vec<String>, FastaUnpackingError> {
                if line.starts_with('>') {
                    acc.push(line.clone());
                } else {
                    let unpacked_line = unpack_string(&line, &mode)?; 
                    acc.push(unpacked_line);
                }
                Ok(acc)
            },
        ).try_reduce(
            || Vec::new(),
            |mut acc, x| -> Result<Vec<String>, FastaUnpackingError> {
                acc.extend(x);
                Ok(acc)
            },
        );

    let mut output_file = BufWriter::new(File::create(output_file_name)?);
    for line in output_lines? {
        writeln!(output_file, "{}", line)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DNA_TEST_MODE: Mode = Mode::DNA;
    const RNA_TEST_MODE: Mode = Mode::RNA;
    
    #[test]
    fn test_dna_compress_string() {
        let test_strings = vec!["AAAA", "AC", "AAAACCCGTT", "AGGGGCCCCTTTTAA", ""];
        for s in test_strings {
            let compressed = compress_string(s, &DNA_TEST_MODE).unwrap();
            let expected_len = (s.len() + 2) / 3;
            assert_eq!(compressed.len(), expected_len);
        }
    }

    #[test]
    fn test_dna_unpack_string() {
        let test_strings = vec!["A", "Aq1", "123", "5", ""];
        for s in test_strings {
            let unpacked = unpack_string(s, &DNA_TEST_MODE).unwrap();
            assert!(unpacked.len() >= s.len());
        }
    }

    #[test]
    fn test_dna_compress_to_file() {
        let input_strings = vec!["AAAA", "AC", "AAAACCCGTT", "AGGGGCCCCTTTTAA",""];
        let input_file_name = "test_input_compress_dna.txt";
        let mut input_file = File::create(input_file_name).unwrap();
        for s in &input_strings {
            writeln!(input_file, "{}", s).unwrap();
        }

        let output_file_name = "test_output_compress_dna.txt";
        compress_to_file(input_file_name, output_file_name, &DNA_TEST_MODE).unwrap();

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
    fn test_dna_unpack_from_file() {
        let input_strings = vec!["A4", "AC", "A4C3G1T2", "AG4C4T4A2"];
        let input_file_name = "test_input_unpack_dna.txt";
        let mut input_file = File::create(input_file_name).unwrap();
        for s in &input_strings {
            writeln!(input_file, "{}", s).unwrap();
        }

        let output_file_name = "test_output_unpack_dna.txt";
        unpack_from_file(input_file_name, output_file_name, &DNA_TEST_MODE).unwrap();

        let output_file = File::open(output_file_name).unwrap();
        let reader = BufReader::new(output_file);
        let output_lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();

        for (input, output) in input_strings.iter().zip(output_lines.iter()) {
            assert!(output.len() >= input.len());
        }

        std::fs::remove_file(input_file_name).unwrap();
        std::fs::remove_file(output_file_name).unwrap();
    }

    #[test]
    fn test_rna_compress_string() {
        let test_strings = vec!["AAAA", "AC", "AAANNNACCCGUU", "AGGNNNGGCCCCUUUAA", ""];
        for s in test_strings {
            let compressed = compress_string(s, &RNA_TEST_MODE).unwrap();
            println!("{}, {}", compressed, s);
            let expected_len = (s.len() + 2) / 3;
            assert_eq!(compressed.len(), expected_len);
        }
    }

    #[test]
    fn test_rna_unpack_string() {
        let test_strings = vec!["A", "Aq1", "123", "5", ""];
        for s in test_strings {
            let unpacked = unpack_string(s, &RNA_TEST_MODE).unwrap();
            assert!(unpacked.len() >= s.len());
        }
    }

    #[test]
    fn test_rna_compress_to_file() {
        let input_strings = vec!["AAAA", "AC", "AAAACCCGUU", "AGGNNNGGCCCCUUUUAA",""];
        let input_file_name = "test_input_compress_rna.txt";
        let mut input_file = File::create(input_file_name).unwrap();
        for s in &input_strings {
            writeln!(input_file, "{}", s).unwrap();
        }

        let output_file_name = "test_output_compress_rna.txt";
        compress_to_file(input_file_name, output_file_name, &RNA_TEST_MODE).unwrap();

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
    fn test_rna_unpack_from_file() {
        let input_strings = vec!["A4", "AC", "A4C3G1T2", "AG4C4T4A2"];
        let input_file_name = "test_input_unpack_rna.txt";
        let mut input_file = File::create(input_file_name).unwrap();
        for s in &input_strings {
            writeln!(input_file, "{}", s).unwrap();
        }

        let output_file_name = "test_output_unpack_rna.txt";
        unpack_from_file(input_file_name, output_file_name, &RNA_TEST_MODE).unwrap();

        let output_file = File::open(output_file_name).unwrap();
        let reader = BufReader::new(output_file);
        let output_lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();

        for (input, output) in input_strings.iter().zip(output_lines.iter()) {
            assert!(output.len() >= input.len());
        }

        std::fs::remove_file(input_file_name).unwrap();
        std::fs::remove_file(output_file_name).unwrap();
    }
    
    #[test]
    fn test_dna_compress_fasta_to_file() {
        let input_strings = vec![(">desc1", "AAAA"), (">desc2", "AC"), (">desc3", "AAAACCCGTT"), (">desc4", "AGGGGCCCCTTTTAA")];
        let input_file_name = "test_input_compress_dna.fasta";
        let mut input_file = File::create(input_file_name).unwrap();
        for (desc, seq) in &input_strings {
            writeln!(input_file, "{}\n{}", desc, seq).unwrap();
        }

        let output_file_name = "test_output_compress_dna.fasta";
        compress_fasta_to_file(input_file_name, output_file_name, &DNA_TEST_MODE).unwrap();

        let output_file = File::open(output_file_name).unwrap();
        let reader = BufReader::new(output_file);
        let mut output_lines = reader.lines().map(|line| line.unwrap());

        for (desc, seq) in &input_strings {
            assert_eq!(output_lines.next().unwrap(), *desc);

            let expected_seq_compressed = compress_string(seq, &DNA_TEST_MODE).unwrap();
            assert_eq!(output_lines.next().unwrap(), expected_seq_compressed);
        }

        std::fs::remove_file(input_file_name).unwrap();
        std::fs::remove_file(output_file_name).unwrap();
    }

    #[test]
    fn test_rna_unpack_fasta_from_file() {
        let input_strings = vec![(">desc1", "A4"), (">desc2", "AC"), (">desc3", "A4C3G1T2"), (">desc4", "AG4C4T4A2")];
        let input_file_name = "test_input_unpack_rna.fasta";
        let mut input_file = File::create(input_file_name).unwrap();
        for (desc, seq) in &input_strings {
            writeln!(input_file, "{}\n{}", desc, seq).unwrap();
        }

        let output_file_name = "test_output_unpack_rna.fasta";
        unpack_fasta_from_file(input_file_name, output_file_name, &RNA_TEST_MODE).unwrap();

        let output_file = File::open(output_file_name).unwrap();
        let reader = BufReader::new(output_file);
        let mut output_lines = reader.lines().map(|line| line.unwrap());

        for (desc, seq) in &input_strings {
            assert_eq!(output_lines.next().unwrap(), *desc);

            let expected_seq_unpacked = unpack_string(seq, &RNA_TEST_MODE).unwrap();
            assert_eq!(output_lines.next().unwrap(), expected_seq_unpacked);
        }

        std::fs::remove_file(input_file_name).unwrap();
        std::fs::remove_file(output_file_name).unwrap();
    }
}