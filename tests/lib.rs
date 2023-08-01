#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use std::io::BufRead;
    use std::fs::File;
    use std::io::Write;
    use compact_sequence::{
        compress_string, 
        unpack_string, 
        unpack_from_file,
        compress_to_file,
    };

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

    #[test]
    fn test_compress_to_file() {
        let input_file_name = "test_input_compress.txt";
        let mut input_file = File::create(input_file_name).unwrap();
        writeln!(input_file, "AAAA").unwrap();
        writeln!(input_file, "AB").unwrap();
        writeln!(input_file, "AAAABBBCCDAA").unwrap();
        writeln!(input_file, "ABBBBCCCCDDDAA").unwrap();

        let output_file_name = "test_output_compress.txt";
        compress_to_file(input_file_name, output_file_name).unwrap();

        let output_file = File::open(output_file_name).unwrap();
        let reader = BufReader::new(output_file);
        let output_lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();

        let expected_output = vec!["A4", "AB", "A4B3C2DA2", "AB4C4D3A2"];
        assert_eq!(output_lines, expected_output);

        std::fs::remove_file(input_file_name).unwrap();
        std::fs::remove_file(output_file_name).unwrap();
    }

    #[test]
    fn test_unpack_from_file() {
        let input_file_name = "test_input_unpack.txt";
        let mut input_file = File::create(input_file_name).unwrap();
        writeln!(input_file, "A4").unwrap();
        writeln!(input_file, "AB").unwrap();
        writeln!(input_file, "A4B3C2DA2").unwrap();
        writeln!(input_file, "AB4C4D3A2").unwrap();

        let output_file_name = "test_output_unpack.txt";
        unpack_from_file(input_file_name, output_file_name).unwrap();

        let output_file = File::open(output_file_name).unwrap();
        let reader = BufReader::new(output_file);
        let output_lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();

        let expected_output = vec!["AAAA", "AB", "AAAABBBCCDAA", "ABBBBCCCCDDDAA"];
        assert_eq!(output_lines, expected_output);

        std::fs::remove_file(input_file_name).unwrap();
        std::fs::remove_file(output_file_name).unwrap();
    }
}
