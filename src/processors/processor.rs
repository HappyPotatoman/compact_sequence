use crate::Mode;
use crate::{
    compress_to_file, 
    unpack_from_file,
    compress_fasta_to_file,
    unpack_fasta_from_file,
};
use crate::processors::directory_processing::{compress_directory, unpack_directory};

pub trait Processor {
    fn compress(&self, input: &str, output_file_name: &str, mode: &Mode) -> Result<(), Box<dyn std::error::Error>>;
    fn unpack(&self, input: &str, output_file_name: &str, mode: &Mode) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct TextProcessor;

impl Processor for TextProcessor {
    fn compress(&self, input: &str, output_file_name: &str, mode: &Mode) -> Result<(), Box<dyn std::error::Error>> {
        compress_to_file(input, output_file_name, mode)
    }

    fn unpack(&self, input: &str, output_file_name: &str, mode: &Mode) -> Result<(), Box<dyn std::error::Error>> {
        unpack_from_file(input, output_file_name, mode)
    }
}

pub struct DirectoryProcessor {
    supported_extensions: Vec<String>,
}

impl DirectoryProcessor {
    pub fn new(supported_extensions: Vec<String>) -> Self {
        Self { supported_extensions }
    }
}

impl Processor for DirectoryProcessor {
    fn compress(&self, input: &str, output_file_name: &str, mode: &Mode) -> Result<(), Box<dyn std::error::Error>> {
        compress_directory(input, output_file_name, mode, &self.supported_extensions)
            .map_err(|e| e.into())
    }

    fn unpack(&self, input: &str, output_file_name: &str, mode: &Mode) -> Result<(), Box<dyn std::error::Error>> {
        unpack_directory(input, output_file_name, mode, &self.supported_extensions)
            .map_err(|e| e.into())
    }

}

pub struct FastaProcessor;

impl Processor for FastaProcessor {
    fn compress(&self, input: &str, output_file_name: &str, mode: &Mode) -> Result<(), Box<dyn std::error::Error>> {
        compress_fasta_to_file(input, output_file_name, mode)
    }

    fn unpack(&self, input: &str, output_file_name: &str, mode: &Mode) -> Result<(), Box<dyn std::error::Error>> {
        unpack_fasta_from_file(input, output_file_name, mode)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{Read, Write};

    #[test]
    fn test_text_processor_dna_compress() {
        let input_path = "test_dna_compress_input.txt";
        let output_path = "test_dna_compress_output.txt";
        let test_sequence = b"ATGCGN";

        let mut file = File::create(input_path).unwrap();
        file.write_all(test_sequence).unwrap();

        let processor = TextProcessor;
        let mode = Mode::DNA;
        processor.compress(input_path, output_path, &mode).unwrap();

        let mut file = File::open(output_path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        assert!(content.trim().len() < test_sequence.len());

        std::fs::remove_file(input_path).unwrap();
        std::fs::remove_file(output_path).unwrap();
    }

    #[test]
    fn test_text_processor_dna_unpack() {
        let input_path = "test_dna_unpack_input.txt";
        let output_path = "test_dna_unpack_output.txt";
        let test_sequence = b"random_ascii_sequence";

        let mut file = File::create(input_path).unwrap();
        file.write_all(test_sequence).unwrap();

        let processor = TextProcessor;
        let mode = Mode::DNA;
        processor.unpack(input_path, output_path, &mode).unwrap();

        let mut file = File::open(output_path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        assert!(content.trim().len() > test_sequence.len());

        std::fs::remove_file(input_path).unwrap();
        std::fs::remove_file(output_path).unwrap();
    }

    #[test]
    fn test_text_processor_rna_compress() {
        let input_path = "test_rna_compress_input.txt";
        let output_path = "test_rna_compress_output.txt";
        let test_sequence = b"AUGCGN";

        let mut file = File::create(input_path).unwrap();
        file.write_all(test_sequence).unwrap();

        let processor = TextProcessor;
        let mode = Mode::RNA;
        processor.compress(input_path, output_path, &mode).unwrap();

        let mut file = File::open(output_path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        assert!(content.trim().len() < test_sequence.len());

        std::fs::remove_file(input_path).unwrap();
        std::fs::remove_file(output_path).unwrap();
    }

    #[test]
    fn test_text_processor_rna_unpack() {
        let input_path = "test_rna_unpack_input.txt";
        let output_path = "test_rna_unpack_output.txt";
        let test_sequence = b"random_ascii_sequence";

        let mut file = File::create(input_path).unwrap();
        file.write_all(test_sequence).unwrap();

        let processor = TextProcessor;
        let mode = Mode::RNA;
        processor.unpack(input_path, output_path, &mode).unwrap();

        let mut file = File::open(output_path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        assert!(content.trim().len() > test_sequence.len());

        std::fs::remove_file(input_path).unwrap();
        std::fs::remove_file(output_path).unwrap();
    }
}