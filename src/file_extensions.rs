pub fn is_fasta_extension(ext: &str) -> bool {
    ["fasta", "fa", "fas", "fna"].contains(&ext)
}

pub fn is_text_extension(ext: &str) -> bool {
    ["txt"].contains(&ext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_fasta_extension() {
        assert!(is_fasta_extension("fasta"));
        assert!(is_fasta_extension("fa"));
        assert!(is_fasta_extension("fas"));
        assert!(is_fasta_extension("fna"));
        assert!(!is_fasta_extension("txt"));
        assert!(!is_fasta_extension("png"));
    }

    #[test]
    fn test_is_text_extension() {
        assert!(is_text_extension("txt"));
        assert!(!is_text_extension("fasta"));
        assert!(!is_text_extension("docx"));
    }
}
