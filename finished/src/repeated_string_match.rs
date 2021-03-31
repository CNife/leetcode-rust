pub fn repeated_string_match(a: String, b: String) -> i32 {
    let mut buffer = String::with_capacity(3 * a.len() + b.len());
    while buffer.len() < b.len() {
        buffer.push_str(&a);
    }
    while buffer.len() <= 2 * a.len() + b.len() {
        if buffer.contains(&b) {
            return (buffer.len() / a.len()) as i32;
        } else {
            buffer.push_str(&a);
        }
    }
    -1
}

#[test]
fn test() {
    let cases = vec![
        ("abcd", "cdabcdab", 3),
        ("abc", "cabcabca", 4),
        ("aaaaaaaaaaaaaaaaaaaaaab", "ba", 2),
    ];
    for (a, b, expect) in cases {
        assert_eq!(repeated_string_match(a.into(), b.into()), expect);
    }
}
