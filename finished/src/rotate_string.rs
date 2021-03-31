pub fn rotate_string(src: String, tested: String) -> bool {
    if src.len() != tested.len() {
        return false;
    }

    let mut double_src = String::with_capacity(src.len() * 2);
    double_src.push_str(&src);
    double_src.push_str(&src);
    double_src.contains(&tested)
}

#[test]
fn test() {
    let cases = vec![
        ("abcde", "cdeab", true),
        ("abcde", "abced", false),
        ("aa", "a", false),
    ];
    for (src, tested, expect) in cases {
        assert_eq!(rotate_string(src.into(), tested.into()), expect);
    }
}
