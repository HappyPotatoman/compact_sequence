use std::collections::HashMap;

pub fn create_encoding_map() -> HashMap<String, String> {
    let mut encoding_map = HashMap::new();

    let bases = ['A', 'G', 'C', 'T'];
    let mut counter = 32; // Start from ' ' (32)

    for &base1 in &bases {
        for &base2 in &bases {
            for &base3 in &bases {
                let triplet = [base1, base2, base3];
                let key = triplet.iter().collect::<String>();
                let encoded_value = std::char::from_u32(counter).unwrap_or('_').to_string();
                encoding_map.insert(key, encoded_value);
                counter += 1;
            }
        }
    }

    for &base1 in &bases {
        for &base2 in &bases {
            let duplet = [base1, base2];
            let key = duplet.iter().collect::<String>();
            let encoded_value = std::char::from_u32(counter).unwrap_or('_').to_string();
            encoding_map.insert(key, encoded_value);
            counter += 1;
        }
    }

    for &base in &bases {
        let key = base.to_string();
        let encoded_value = std::char::from_u32(counter).unwrap_or('_').to_string();
        encoding_map.insert(key, encoded_value);
        counter += 1;
    }

    // Insert special singlet
    let key = "I".to_string();
    let encoded_value = std::char::from_u32(counter).unwrap_or('_').to_string();
    encoding_map.insert(key, encoded_value);

    encoding_map
}