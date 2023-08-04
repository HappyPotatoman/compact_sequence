use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::fs::File;

#[test]
fn test_compress_and_unpack() {
    let test_input = "AAAGGGCCCTTT";
    let input_file_name = "test_input.txt";
    let compressed_file_name = "compress_and_unpack_test_compressed.txt";
    let unpacked_file_name = "compress_and_unpack_test_unpacked.txt";

    {
        let mut file = File::create(input_file_name).unwrap();
        writeln!(file, "{}", test_input).unwrap();
    }


    compact_sequence::compress_to_file(input_file_name, compressed_file_name).unwrap();
    
    compact_sequence::unpack_from_file(compressed_file_name, unpacked_file_name).unwrap();

    let unpacked_file = File::open(unpacked_file_name).unwrap();
    let reader = BufReader::new(unpacked_file);
    let unpacked: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();

    assert_eq!(test_input, unpacked.join(""));

    std::fs::remove_file(compressed_file_name).unwrap();
    std::fs::remove_file(unpacked_file_name).unwrap();
    std::fs::remove_file(input_file_name).unwrap();
}
