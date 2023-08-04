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
        let test_strings = vec!["AAAA", "AC", "AAAACCCGTT", "AGGGGCCCCTTTTAA", ""];
        for s in test_strings {
            let compressed = compress_string(s).unwrap();
            let expected_len = (s.len() + 2) / 3;
            assert_eq!(compressed.len(), expected_len);
        }
    }

    #[test]
    fn test_unpack_string() {
        let test_strings = vec!["A", "Aq1", "123", "5", ""];
        for s in test_strings {
            let unpacked = unpack_string(s).unwrap();
            assert!(unpacked.len() >= s.len());
        }
    }

    #[test]
fn test_compress_to_file() {
    let input_strings = vec!["AAAA", "AC", "AAAACCCGTT", "AGGGGCCCCTTTTAA"];
    let input_file_name = "test_input_compress.txt";
    let mut input_file = File::create(input_file_name).unwrap();
    for s in &input_strings {
        writeln!(input_file, "{}", s).unwrap();
    }

    let output_file_name = "test_output_compress.txt";
    compress_to_file(input_file_name, output_file_name).unwrap();

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
fn test_unpack_from_file() {
    let input_strings = vec!["A4", "AC", "A4C3G1T2", "AG4C4T4A2"];
    let input_file_name = "test_input_unpack.txt";
    let mut input_file = File::create(input_file_name).unwrap();
    for s in &input_strings {
        writeln!(input_file, "{}", s).unwrap();
    }

    let output_file_name = "test_output_unpack.txt";
    unpack_from_file(input_file_name, output_file_name).unwrap();

    let output_file = File::open(output_file_name).unwrap();
    let reader = BufReader::new(output_file);
    let output_lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();

    for (input, output) in input_strings.iter().zip(output_lines.iter()) {
        assert!(output.len() >= input.len());
    }

    std::fs::remove_file(input_file_name).unwrap();
    std::fs::remove_file(output_file_name).unwrap();
}
}

