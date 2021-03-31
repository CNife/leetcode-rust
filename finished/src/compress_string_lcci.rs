use std::fmt::Write;

pub fn compress_string(input: String) -> String {
    let mut slice = input.as_str();
    let mut compressed = String::new();
    while let Some((ch, len)) = compress(slice) {
        slice = &slice[len..];
        write!(&mut compressed, "{}{}", ch, len).unwrap();
    }

    if compressed.len() < input.len() {
        compressed
    } else {
        input
    }
}

fn compress(input: &str) -> Option<(char, usize)> {
    input.chars().next().map(|first| {
        let count = input.chars().take_while(|ch| *ch == first).count();
        (first, count)
    })
}

#[test]
fn test() {
    let cases = vec![("aabcccccaaa", "a2b1c5a3"), ("abbccd", "abbccd")];
    for (input, expected) in cases {
        assert_eq!(compress_string(input.to_string()), expected);
    }
}
