pub fn longest_palindrome(s: String) -> String {
    if s.is_empty() {
        return s;
    }

    let s = s.into_bytes();
    let mut max_len = 1;
    let mut res_left = 0;
    let mut res_right = 0;

    let mut helper = |left: usize, right: usize| {
        let (l, r) = expand_palindrome(&s, left, right);
        let len = r - l + 1;
        if len > max_len {
            max_len = len;
            res_left = l;
            res_right = r;
        }
    };

    for i in 0..s.len() - 1 {
        helper(i, i);
        helper(i, i + 1);
    }
    unsafe { String::from_utf8_unchecked(s[res_left..=res_right].to_owned()) }
}

fn expand_palindrome(s: &[u8], mut left: usize, mut right: usize) -> (usize, usize) {
    loop {
        if s[left] != s[right] {
            return if left + 1 == right {
                (left, left)
            } else {
                (left + 1, right - 1)
            };
        }
        if left == 0 || right == s.len() - 1 {
            return (left, right);
        }
        left -= 1;
        right += 1;
    }
}

#[test]
fn test() {
    let is_palindrome = |s: &str| -> bool {
        s.chars()
            .zip(s.chars().rev())
            .take(s.len() / 2)
            .all(|(c1, c2)| c1 == c2)
    };

    let cases: Vec<(&str, usize)> = vec![("babad", 3), ("cbbd", 2), ("", 0)];
    for (input, expected_len) in cases {
        let output = longest_palindrome(input.to_string());
        assert_eq!(output.len(), expected_len);
        assert!(is_palindrome(&output));
        assert!(input.contains(&output));
    }
}
