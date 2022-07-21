use std::collections::HashMap;

pub fn find_repeated_dna_sequences(s: impl AsRef<str>) -> Vec<String> {
    let bytes = s.as_ref().as_bytes();
    let n = bytes.len();
    if n < 10 {
        return vec![];
    }

    let mut pattern = 0u32;
    let mut pattern_map: HashMap<u32, i32> = HashMap::new();
    for byte in bytes[..9].iter() {
        add(&mut pattern, *byte);
    }
    for byte in bytes[9..].iter() {
        remove(&mut pattern);
        add(&mut pattern, *byte);
        pattern_map
            .entry(pattern)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    pattern_map
        .into_iter()
        .filter(|&(_, count)| count > 1)
        .map(|(pattern, _)| pattern_to_string(pattern))
        .collect()
}

fn add(pattern: &mut u32, byte: u8) {
    let n: u32 = match byte {
        b'A' => 0,
        b'C' => 1,
        b'G' => 2,
        b'T' => 3,
        _ => panic!("invalid input"),
    };
    *pattern = *pattern * 4 + n;
}

fn remove(pattern: &mut u32) {
    const MASK: u32 = 0x40000;
    *pattern -= (*pattern / MASK) * MASK;
}

fn pattern_to_string(mut pattern: u32) -> String {
    const DNA_DICT: [u8; 4] = [b'A', b'C', b'G', b'T'];
    let mut result = [0u8; 10];
    for i in (0..10).rev() {
        let n = pattern % 4;
        pattern /= 4;
        result[i] = DNA_DICT[n as usize];
    }
    String::from_utf8(result.to_vec()).unwrap()
}

#[test]
fn test_find_repeated_dna_sequences() {
    use utils::assert_same_set;

    let tests = vec![
        (
            "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT",
            vec!["AAAAACCCCC", "CCCCCAAAAA"],
        ),
        ("AAAAAAAAAAAAA", vec!["AAAAAAAAAA"]),
    ];
    for (s, want) in tests {
        assert_same_set(find_repeated_dna_sequences(s), want);
    }
}
