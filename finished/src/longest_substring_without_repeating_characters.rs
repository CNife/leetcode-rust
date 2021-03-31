use std::cmp::max;
use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let s = s.into_bytes();
    let mut positions = HashMap::new();
    let mut start = 0;
    let mut end = 0;
    let mut result = 0;

    while end < s.len() {
        if let Some(next_start) = positions.get(&s[end]) {
            result = max(result, end - start);
            start = max(start, next_start + 1);
        }
        positions.insert(s[end], end);
        end += 1;
    }
    max(result, end - start) as i32
}

#[test]
fn test() {
    let cases = vec![
        ("abcabcbb", 3),
        ("bbbbb", 1),
        ("pwwkew", 3),
        (" ", 1),
        ("abba", 2),
    ];
    for (s, expected) in cases {
        assert_eq!(length_of_longest_substring(s.to_string()), expected);
    }
}
