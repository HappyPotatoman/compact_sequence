use std::collections::HashMap;

lazy_static::lazy_static! {
        pub static ref ENCODING_MAP: HashMap<String, String> = create_encoding_map();
        pub static ref DECODING_MAP: HashMap<String, String> = create_decoding_map();
    }

pub fn create_encoding_map() -> HashMap<String, String> {
    let mut encoding_map = HashMap::new();

    let bases = ['A', 'G', 'C', 'T'];
    let mut counter = 33; 

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

    let key = "I".to_string();
    let encoded_value = std::char::from_u32(counter).unwrap_or('_').to_string();
    encoding_map.insert(key, encoded_value);
    counter += 1;

    let key = "NNN".to_string();
    let encoded_value = std::char::from_u32(counter).unwrap_or('_').to_string();
    encoding_map.insert(key, encoded_value);
    counter += 1;
    
    let key = "NN".to_string();
    let encoded_value = std::char::from_u32(counter).unwrap_or('_').to_string();
    encoding_map.insert(key, encoded_value);
    counter += 1;

    let key = "N".to_string();
    let encoded_value = std::char::from_u32(counter).unwrap_or('_').to_string();
    encoding_map.insert(key, encoded_value);

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
