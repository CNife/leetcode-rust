pub fn repeated_substring_pattern(s: String) -> bool {
    if s.len() < 2 {
        false
    } else {
        let mut doubled_s = String::with_capacity(s.len() * 2 - 2);
        doubled_s.push_str(&s[1..]);
        doubled_s.push_str(&s[..s.len() - 1]);
        doubled_s.contains(&s)
    }
}

#[test]
fn test() {
    let cases = vec![("abab", true), ("aba", false), ("abcabcabcabc", true)];
    for (s, expected) in cases {
        assert_eq!(repeated_substring_pattern(s.to_string()), expected);
    }
}
