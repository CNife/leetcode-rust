pub fn count_binary_substrings(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();

    let mut result = 0;
    for i in 1..len {
        let mut left = i - 1;
        let mut right = i;
        if s[left] == s[right] {
            continue;
        }

        let left_byte = s[left];
        let right_byte = s[right];
        while left < len && right < len && s[left] == left_byte && s[right] == right_byte {
            left = left.wrapping_sub(1);
            right += 1;
            result += 1;
        }
    }
    result
}

#[test]
fn test() {
    let cases = vec![("00110011", 6), ("10101", 4)];
    for (s, expect) in cases {
        assert_eq!(count_binary_substrings(s.into()), expect);
    }
}
