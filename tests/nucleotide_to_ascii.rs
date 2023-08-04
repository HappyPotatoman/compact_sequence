use compact_sequence::encoders::nucleotide_to_ascii;

#[test]
fn test_create_encoding_map() {
    let map = nucleotide_to_ascii::create_encoding_map();

    assert_eq!(map.len(), 85);

    let bases = ['A', 'G', 'C', 'T'];

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
    assert!(map.contains_key("I"));

    let values: std::collections::HashSet<_> = map.values().collect();
    assert_eq!(values.len(), map.len());

    for value in map.values() {
        let chars: Vec<char> = value.chars().collect();
        assert!(chars.len() == 1);
        let ascii_value = chars[0] as u32;
        assert!(ascii_value >= 32 && ascii_value <= 126);
    }
}

#[test]
fn test_create_decoding_map() {
    let encoding_map = nucleotide_to_ascii::create_encoding_map();
    let decoding_map = nucleotide_to_ascii::create_decoding_map();

    assert_eq!(decoding_map.get(&encoding_map["AAA"]), Some(&"AAA".to_string()));
    assert_eq!(decoding_map.get(&encoding_map["AGT"]), Some(&"AGT".to_string()));
    assert_eq!(decoding_map.get(&encoding_map["GCC"]), Some(&"GCC".to_string()));

    assert_eq!(decoding_map.get(&encoding_map["AG"]), Some(&"AG".to_string()));
    assert_eq!(decoding_map.get(&encoding_map["CT"]), Some(&"CT".to_string()));

    assert_eq!(decoding_map.get(&encoding_map["A"]), Some(&"A".to_string()));
    assert_eq!(decoding_map.get(&encoding_map["G"]), Some(&"G".to_string()));

    assert_eq!(decoding_map.get(&encoding_map["I"]), Some(&"I".to_string()));

    assert_eq!(decoding_map.get("z"), None);
}