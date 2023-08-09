use std::collections::HashMap;

lazy_static::lazy_static! {
    pub static ref RNA_ENCODING_MAP: std::collections::HashMap<String, String> = rna_encoding_map();
    pub static ref RNA_DECODING_MAP: std::collections::HashMap<String, String> = rna_decoding_map();
}

pub fn rna_encoding_map() -> HashMap<String, String> {
    create_encoding_map()
}

pub fn rna_decoding_map() -> HashMap<String, String> {
    create_decoding_map()
}

fn create_encoding_map() -> HashMap<String, String> {
    let mut encoding_map = HashMap::new();

    let bases = ['A', 'G', 'C', 'U', 'N'];
    let mut non_n_counter = 34; 
    let mut n_counter = 34;

    for &base1 in &bases {
        for &base2 in &bases {
            for &base3 in &bases {
                let triplet = [base1, base2, base3];
                let key = triplet.iter().collect::<String>();
                if key.contains('N') && key != "NNN" {
                    let encoded_value = format!("!{}", std::char::from_u32(n_counter).unwrap_or('_'));
                    encoding_map.insert(key, encoded_value);
                    n_counter += 1;
                    if n_counter == 62 {
                        n_counter += 1
                    }
                } else {
                    let encoded_value = std::char::from_u32(non_n_counter).unwrap_or('_').to_string();
                    encoding_map.insert(key, encoded_value);
                    non_n_counter += 1;
                    if non_n_counter == 62 {
                        non_n_counter += 1
                    }
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
                if n_counter == 62 {
                    n_counter += 1
                }
            } else {
                let encoded_value = std::char::from_u32(non_n_counter).unwrap_or('_').to_string();
                encoding_map.insert(key, encoded_value);
                non_n_counter += 1;
                if non_n_counter == 62 {
                    non_n_counter += 1
                }
            }
        }
    }

    for &base in &bases {
        let key = base.to_string();
        if key.contains('N') {
            let encoded_value = format!("!{}", std::char::from_u32(n_counter).unwrap_or('_'));
            encoding_map.insert(key, encoded_value);
            n_counter += 1;
            if n_counter == 62 {
                n_counter += 1
            }
        } else {
            let encoded_value = std::char::from_u32(non_n_counter).unwrap_or('_').to_string();
            encoding_map.insert(key, encoded_value);
            non_n_counter += 1;
            if non_n_counter == 62 {
                non_n_counter += 1
            }
        }
    }

    encoding_map
}

fn create_decoding_map() -> HashMap<String, String> {
    let encoding_map = create_encoding_map();
    let mut decoding_map = HashMap::new();

    for (key, value) in encoding_map {
        decoding_map.insert(value, key);
    }

    decoding_map
}

#[cfg(test)]
mod tests {

    use crate::encoders::rna_to_ascii;

    #[test]
    fn test_create_encoding_map() {
        let map = rna_to_ascii::create_encoding_map();

        assert_eq!(map.len(), 155);

        let bases = ['A', 'G', 'C', 'U', 'N'];

        for &base1 in &bases {
            for &base2 in &bases {
                for &base3 in &bases {
                    let triplet = format!("{}{}{}", base1, base2, base3);
                    assert!(map.contains_key(&triplet));
                }

                let duplet = format!("{}{}", base1, base2);
                assert!(map.contains_key(&duplet));
            }

            let singlet = base1.to_string();
            assert!(map.contains_key(&singlet));
        }

        let values: std::collections::HashSet<_> = map.values().collect();
        assert_eq!(values.len(), map.len());

        for value in map.values() {
            let chars: Vec<char> = value.chars().collect();
            let value_string = chars.iter().collect::<String>();
            assert!(chars.len() == 1 || value_string.starts_with("!"));
            let ascii_value = chars[0] as u32;
            assert!(ascii_value >= 32 && ascii_value <= 126);
        }
    }

    #[test]
    fn test_create_decoding_map() {
        let encoding_map = rna_to_ascii::create_encoding_map();
        let decoding_map = rna_to_ascii::create_decoding_map();

        assert_eq!(decoding_map.get(&encoding_map["AAA"]), Some(&"AAA".to_string()));
        assert_eq!(decoding_map.get(&encoding_map["AGU"]), Some(&"AGU".to_string()));
        assert_eq!(decoding_map.get(&encoding_map["GCC"]), Some(&"GCC".to_string()));

        assert_eq!(decoding_map.get(&encoding_map["ANA"]), Some(&"ANA".to_string()));
        assert_eq!(decoding_map.get(&encoding_map["AGN"]), Some(&"AGN".to_string()));
        assert_eq!(decoding_map.get(&encoding_map["NNN"]), Some(&"NNN".to_string()));

        assert_eq!(decoding_map.get(&encoding_map["AG"]), Some(&"AG".to_string()));
        assert_eq!(decoding_map.get(&encoding_map["CU"]), Some(&"CU".to_string()));

        assert_eq!(decoding_map.get(&encoding_map["NG"]), Some(&"NG".to_string()));
        assert_eq!(decoding_map.get(&encoding_map["CN"]), Some(&"CN".to_string()));

        assert_eq!(decoding_map.get(&encoding_map["A"]), Some(&"A".to_string()));
        assert_eq!(decoding_map.get(&encoding_map["G"]), Some(&"G".to_string()));
        assert_eq!(decoding_map.get(&encoding_map["U"]), Some(&"U".to_string()));
        
        assert!(!encoding_map.contains_key(&"T".to_string()));

        assert_eq!(decoding_map.get(&encoding_map["N"]), Some(&"N".to_string()));

        assert_eq!(decoding_map.get("z"), None);
    }
    #[test]
    fn test_no_greater_than_character_in_keys() {
        let encoding_map = rna_to_ascii::create_encoding_map();
        let decoding_map = rna_to_ascii::create_decoding_map();
        for key in encoding_map.keys() {
            assert!(!key.contains('>'), "The key '{}' should not contain the character '>'", key);
        }
        for key in decoding_map.keys() {
            assert!(!key.contains('>'), "The key '{}' should not contain the character '>'", key);
        }
    }
}