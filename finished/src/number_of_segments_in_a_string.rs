pub fn count_segments(s: String) -> i32 {
    s.split_ascii_whitespace().count() as i32
}

#[test]
fn test() {
    assert_eq!(count_segments("Hello, my name is John".to_string()), 5);
}
