pub fn reverse_words(input: String) -> String {
    let mut word_iter = input.split_ascii_whitespace().rev();
    match word_iter.next() {
        None => String::new(),
        Some(first) => {
            let mut result = String::from(first);
            for word in word_iter {
                result.push(' ');
                result.push_str(word);
            }
            result
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        ("the sky is blue", "blue is sky the"),
        ("  hello world!  ", "world! hello"),
        ("a good   example", "example good a"),
    ];
    for (input, expect) in cases {
        assert_eq!(reverse_words(input.into()), expect);
    }
}
