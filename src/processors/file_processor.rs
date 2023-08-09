use crate::Mode;
use crate::{compress_to_file, unpack_from_file};

pub trait FileProcessor {
    fn compress(&self, input: &str, output_file_name: &str, mode: &Mode) -> Result<(), Box<dyn std::error::Error>>;
    fn unpack(&self, input: &str, output_file_name: &str, mode: &Mode) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct TextProcessor;

impl FileProcessor for TextProcessor {
    fn compress(&self, input: &str, output_file_name: &str, mode: &Mode) -> Result<(), Box<dyn std::error::Error>> {
        compress_to_file(input, output_file_name, mode)
    }

    fn unpack(&self, input: &str, output_file_name: &str, mode: &Mode) -> Result<(), Box<dyn std::error::Error>> {
        unpack_from_file(input, output_file_name, mode)
    }
}