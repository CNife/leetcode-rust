pub struct Solution;

impl Solution {
    pub fn find_longest_word(s: String, d: Vec<String>) -> String {
        let mut result = "".to_string();
        for word in d {
            if Solution::in_word(&s, &word)
                && (word.len() > result.len() || word.len() == result.len() && word < result)
            {
                result = word;
            }
        }
        result
    }

    fn in_word(src: &str, word: &str) -> bool {
        let src = src.as_bytes();
        let word = word.as_bytes();
        let mut src_idx = 0usize;
        let mut word_idx = 0usize;

        while src_idx < src.len() && word_idx < word.len() {
            if word[word_idx] == src[src_idx] {
                word_idx += 1;
            }
            src_idx += 1;
        }
        word_idx == word.len()
    }
}

#[test]
pub fn test_find_longest_word() {
    let cases = vec![
        ("abpcplea", vec!["ale", "apple", "monkey", "plea"], "apple"),
        ("abpcplea", vec!["a", "b", "c"], "a"),
    ];

    for (string, dict, expected) in cases {
        let string = string.to_string();
        let dict: Vec<_> = dict.into_iter().map(|s| s.to_string()).collect();
        let result = Solution::find_longest_word(string, dict);
        assert_eq!(result, expected.to_string());
    }
}
