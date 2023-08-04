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