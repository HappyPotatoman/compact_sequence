


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

    // Handle the last character group
    if char_count > 1 {
        compressed.push_str(&format!("{}{}", current_char, char_count));
    } else if char_count == 1 {
        compressed.push(current_char);
    }

    compressed
}
