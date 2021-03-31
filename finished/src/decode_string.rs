pub fn decode_string(input: String) -> String {
    let result = do_decode_string(input.as_bytes(), 0).1;
    String::from_utf8(result).unwrap()
}

fn do_decode_string(input: &[u8], mut i: usize) -> (usize, Vec<u8>) {
    let mut result = Vec::new();
    let mut count = 0i32;
    while i < input.len() {
        let ch = input[i];
        match ch {
            b'0'..=b'9' => {
                count = count * 10 + (ch - b'0') as i32;
            }
            b'[' => {
                let (next, inner) = do_decode_string(input, i + 1);
                for _ in 0..count {
                    result.extend_from_slice(&inner);
                }
                count = 0;
                i = next;
            }
            b']' => return (i, result),
            _ => {
                result.push(ch);
            }
        }
        i += 1;
    }
    (i + 1, result)
}

#[test]
fn test() {
    let cases = vec![
        ("3[a]2[bc]", "aaabcbc"),
        ("3[a2[c]]", "accaccacc"),
        ("2[abc]3[cd]ef", "abcabccdcdcdef"),
    ];
    for (input, expected) in cases {
        assert_eq!(decode_string(input.to_string()), expected.to_string());
    }
}
