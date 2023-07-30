#[cfg(test)]
mod tests {
    use compact_sequence::compress_string;

    #[test]
    fn test_compress_string() {
        assert_eq!(compress_string("AAAA"), "A4");
        assert_eq!(compress_string("AB"), "AB");
        assert_eq!(compress_string("AAAABBBCCDAA"), "A4B3C2DA2");
        assert_eq!(compress_string("ABBBBCCCCDDDAA"), "AB4C4D3A2");
        assert_eq!(compress_string(""), "");
    }
}
