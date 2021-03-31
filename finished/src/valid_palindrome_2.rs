pub fn valid_palindrome(input: String) -> bool {
    let input = input.as_bytes();
    //noinspection ALL
    fn is_palindrome(string: &[u8]) -> bool {
        string
            .iter()
            .zip(string.iter().rev())
            .take(string.len() / 2)
            .all(|(lhs, rhs)| lhs == rhs)
    }

    for i in 0..input.len() / 2 {
        let j = input.len() - 1 - i;
        if input[i] != input[j] {
            return is_palindrome(&input[i + 1..=j]) || is_palindrome(&input[i..j]);
        }
    }
    true
}

#[test]
fn test() {
    let cases = vec![("aba", true), ("abca", true)];
    for (input, expect) in cases {
        assert_eq!(valid_palindrome(input.into()), expect);
    }
}
