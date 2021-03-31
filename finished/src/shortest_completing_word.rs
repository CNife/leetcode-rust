use std::collections::BinaryHeap;

pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
    let proto_id = string_id(license_plate.as_str());
    let heap: BinaryHeap<(usize, usize)> = words
        .iter()
        .enumerate()
        .filter(|(_, word)| {
            let id = string_id(word.as_str());
            Iterator::zip(id.iter(), proto_id.iter()).all(|(lhs, rhs)| lhs >= rhs)
        })
        .map(|(index, word)| (usize::MAX - word.len(), usize::MAX - index))
        .collect();
    words[usize::MAX - heap.peek().unwrap().1].clone()
}

fn string_id(string: &str) -> [u8; 26] {
    let mut id_array = [0u8; 26];
    for ch in string.chars() {
        if ch.is_ascii_alphabetic() {
            let index = ch.to_ascii_lowercase() as u8 - b'a';
            id_array[index as usize] += 1;
        }
    }
    id_array
}

#[test]
fn test() {
    use utils::vec_of;

    let cases = vec![
        (
            "1s3 PSt",
            vec!["step", "steps", "stripe", "stepple"],
            "steps",
        ),
        ("1s3 456", vec!["looks", "pest", "stew", "show"], "pest"),
    ];
    for (license_plate, words, expect) in cases {
        assert_eq!(
            shortest_completing_word(license_plate.into(), vec_of(words)),
            expect
        );
    }
}
