pub fn license_key_formatting(s: String, k: i32) -> String {
    let s = s.into_bytes();
    let mut result = Vec::with_capacity(s.len());
    let mut i = 0;
    for ch in s.into_iter().rev() {
        if ch != b'-' {
            result.push(ch.to_ascii_uppercase());
            i += 1;
            if i == k {
                result.push(b'-');
                i = 0;
            }
        }
    }
    if let Some(&b'-') = result.last() {
        result.pop();
    }
    result.reverse();
    unsafe { String::from_utf8_unchecked(result) }
}

#[test]
fn test() {
    let cases = vec![
        ("5F3Z-2e-9-w", 4, "5F3Z-2E9W"),
        ("2-5g-3-J", 2, "2-5G-3J"),
        ("---", 3, ""),
    ];
    for (s, k, expected) in cases {
        assert_eq!(license_key_formatting(s.to_string(), k), expected);
    }
}
