pub fn is_subsequence(s: String, t: String) -> bool {
    let mut s_iter = s.bytes().peekable();
    let mut t_iter = t.bytes();
    while let (Some(&s_ch), Some(t_ch)) = (s_iter.peek(), t_iter.next()) {
        if s_ch == t_ch {
            s_iter.next().unwrap();
        }
    }
    s_iter.next().is_none()
}

#[test]
fn test() {
    let tests = vec![("abc", "ahbgdc", true), ("axc", "ahbgdc", false)];
    for (s, t, want) in tests {
        assert_eq!(is_subsequence(s.into(), t.into()), want);
    }
}
