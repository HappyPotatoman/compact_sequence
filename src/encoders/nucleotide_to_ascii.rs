use std::collections::HashMap;

lazy_static::lazy_static! {
    pub static ref ENCODING_MAP: HashMap<String, String> = create_encoding_map();
    pub static ref DECODING_MAP: HashMap<String, String> = create_decoding_map();
}

pub fn create_encoding_map() -> HashMap<String, String> {
    let mut encoding_map = HashMap::new();

    let bases = ['A', 'G', 'C', 'T', 'N'];
    let mut non_n_counter = 34; 
    let mut n_counter = 34;

    for &base1 in &bases {
        for &base2 in &bases {
            for &base3 in &bases {
                let triplet = [base1, base2, base3];
                let key = triplet.iter().collect::<String>();
                if key.contains('N') {
                    let encoded_value = format!("!{}", std::char::from_u32(n_counter).unwrap_or('_'));
                    encoding_map.insert(key, encoded_value);
                    n_counter += 1;
                } else {
                    let encoded_value = std::char::from_u32(non_n_counter).unwrap_or('_').to_string();
                    encoding_map.insert(key, encoded_value);
                    non_n_counter += 1;
                }
            }
        }
    }

    for &base1 in &bases {
        for &base2 in &bases {
            let duplet = [base1, base2];
            let key = duplet.iter().collect::<String>();
            if key.contains('N') {
                let encoded_value = format!("!{}", std::char::from_u32(n_counter).unwrap_or('_'));
                encoding_map.insert(key, encoded_value);
                n_counter += 1;
            } else {
                let encoded_value = std::char::from_u32(non_n_counter).unwrap_or('_').to_string();
                encoding_map.insert(key, encoded_value);
                non_n_counter += 1;
            }
        }
    }

    for &base in &bases {
        let key = base.to_string();
        if key.contains('N') {
            let encoded_value = format!("!{}", std::char::from_u32(n_counter).unwrap_or('_'));
            encoding_map.insert(key, encoded_value);
            n_counter += 1;
        } else {
            let encoded_value = std::char::from_u32(non_n_counter).unwrap_or('_').to_string();
            encoding_map.insert(key, encoded_value);
            non_n_counter += 1;
        }
    }

    encoding_map
}

pub fn create_decoding_map() -> HashMap<String, String> {
    let encoding_map = create_encoding_map();
    let mut decoding_map = HashMap::new();

    for (key, value) in encoding_map {
        decoding_map.insert(value, key);
    }

    decoding_map
}