#[cfg(test)]
mod tests {
    use compact_sequence::{compress_string, unpack_string};

    #[test]
    fn test_compress_string() {
        assert_eq!(compress_string("AAAA"), "A4");
        assert_eq!(compress_string("AB"), "AB");
        assert_eq!(compress_string("AAAABBBCCDAA"), "A4B3C2DA2");
        assert_eq!(compress_string("ABBBBCCCCDDDAA"), "AB4C4D3A2");
        assert_eq!(compress_string(""), "");
    }

    #[test]
    fn test_unpack_string() {
        assert_eq!(unpack_string("A4"), "AAAA");
        assert_eq!(unpack_string("AB"), "AB");
        assert_eq!(unpack_string("A4B3C2DA2"), "AAAABBBCCDAA");
        assert_eq!(unpack_string("AB4C4D3A2"), "ABBBBCCCCDDDAA");
        assert_eq!(unpack_string(""), "");
    }
}
