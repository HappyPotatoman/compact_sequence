use std::fs::File;
use std::io::Write;
use std::io::BufReader;
use std::io::BufRead;


pub fn compress_string(input: &str) -> String {
    let mut compressed = String::new();
    let mut current_char = '_';
    let mut char_count = 0;

    for ch in input.chars() {
        if ch == current_char {
            char_count += 1;
        } else {
            if char_count > 1 {
                compressed.push_str(&format!("{}{}", current_char, char_count));
            } else if char_count == 1 {
                compressed.push(current_char);
            }
            current_char = ch;
            char_count = 1;
        }
    }

    if char_count > 1 {
        compressed.push_str(&format!("{}{}", current_char, char_count));
    } else if char_count == 1 {
        compressed.push(current_char);
    }

    compressed
}

pub fn unpack_string(input: &str) -> String {
    let mut unpacked = String::new();
    let mut i = 0;

    while i < input.len() {
        let ch = input.chars().nth(i).unwrap();
        i += 1;
        
        let mut num_chars = String::new();
        while i < input.len() && input.chars().nth(i).unwrap().is_digit(10) {
            num_chars.push(input.chars().nth(i).unwrap());
            i += 1;
        }

        let count: usize = num_chars.parse().unwrap_or(1);

        for _ in 0..count {
            unpacked.push(ch);
        }
    }
    unpacked
}

pub fn compress_to_file(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input_file = File::open(input)?;
    let reader = BufReader::new(input_file);

    let mut output_file = File::create("output.txt")?;

    for line in reader.lines() {
        let line = line?;
        let compressed_line = compress_string(&line);
        writeln!(output_file, "{}", compressed_line)?;
    }

    Ok(())
}

pub fn unpack_from_file(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input_file = File::open(input)?;
    let reader = BufReader::new(input_file);

    let mut output_file = File::create("output.txt")?;

    for line in reader.lines() {
        let line = line?;
        let unpacked_line = unpack_string(&line);
        writeln!(output_file, "{}", unpacked_line)?;
    }

    Ok(())
}