pub fn is_fasta_extension(ext: &str) -> bool {
    ["fasta", "fa", "fas", "fna"].contains(&ext)
}

pub fn is_text_extension(ext: &str) -> bool {
    ["txt"].contains(&ext)
}