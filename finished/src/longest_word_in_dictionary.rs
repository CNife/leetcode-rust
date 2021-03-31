use std::collections::HashSet;

pub fn longest_word(words: Vec<String>) -> String {
    let word_set: HashSet<&str> = words.iter().map(|s| s.as_str()).collect();
    words
        .iter()
        .filter(|word| (1..word.len()).all(|i| word_set.contains(&word[..i])))
        .max_by(|lhs, rhs| lhs.len().cmp(&rhs.len()).then(rhs.cmp(lhs)))
        .map_or("".into(), |s| s.clone())
}

#[test]
fn test() {
    use utils::vec_of;

    let cases = vec![
        (vec!["w", "wo", "wor", "worl", "world"], "world"),
        (
            vec!["a", "banana", "app", "appl", "ap", "apply", "apple"],
            "apple",
        ),
    ];
    for (words, expect) in cases {
        assert_eq!(longest_word(vec_of(words)), expect);
    }
}
