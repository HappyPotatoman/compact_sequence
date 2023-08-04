use std::fmt;

#[derive(Debug)]
pub enum CompressionError {
    UnknownSequence(String),
    UnknownCharacter(char),
}

impl fmt::Display for CompressionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompressionError::UnknownSequence(seq) => write!(f, "Unknown sequence: {}", seq),
            CompressionError::UnknownCharacter(ch) => write!(f, "Unknown character: {}", ch),
        }
    }
}

impl std::error::Error for CompressionError {}
