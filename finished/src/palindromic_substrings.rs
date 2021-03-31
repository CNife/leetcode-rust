use std::cmp::min;

pub fn count_substrings(s: String) -> i32 {
    let s = s.into_bytes();
    odd_count(&s) + even_count(&s)
}

fn odd_count(s: &[u8]) -> i32 {
    let mut result = s.len() as i32;
    for center in 1..s.len() - 1 {
        for i in 1..=min(center, s.len() - center - 1) {
            if s[center - i] == s[center + i] {
                result += 1;
            } else {
                break;
            }
        }
    }
    result
}

fn even_count(s: &[u8]) -> i32 {
    let mut result = 0;
    for left in 0..s.len() - 1 {
        for i in 0..=min(left, s.len() - left - 2) {
            if s[left - i] == s[left + i + 1] {
                result += 1;
            } else {
                break;
            }
        }
    }
    result
}

//fn can_expand(s: &[u8], start: usize, end: usize) -> bool {
//    start > 0 && end < s.len() && s[start - 1] == s[end + 1]
//}

#[test]
fn test() {
    let cases = vec![("abc", 3), ("aaa", 6)];
    for (s, expected) in cases {
        assert_eq!(count_substrings(s.to_string()), expected);
    }
}
