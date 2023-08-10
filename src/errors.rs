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

#[derive(Debug)]
pub struct FastaCompressionError(Box<dyn std::error::Error + Send>);

impl std::fmt::Display for FastaCompressionError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for FastaCompressionError{}

impl From<CompressionError> for FastaCompressionError{
    fn from(err: CompressionError) -> Self {
        FastaCompressionError(Box::new(err))
    }
}
