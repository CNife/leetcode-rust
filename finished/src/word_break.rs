// 简单回溯，超时
// pub fn word_break(input: String, dict: Vec<String>) -> bool {
//     backtrace(&input, &dict)
// }
//
// fn backtrace(input: &str, dict: &[String]) -> bool {
//     if input.is_empty() {
//         true
//     } else {
//         for word in dict {
//             if input.starts_with(word) && backtrace(&input[word.len()..], dict) {
//                 return true;
//             }
//         }
//         false
//     }
// }

// 简单动态规则，0ms，2.1MB
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn word_break(input: String, dict: Vec<String>) -> bool {
    let n = input.len();
    let dict: HashSet<String> = HashSet::from_iter(dict.into_iter());
    let mut dp = vec![false; input.len() + 1];
    dp[0] = true;
    for i in 0..n {
        for j in i + 1..n + 1 {
            if dp[i] && dict.contains(&input[i..j]) {
                dp[j] = true;
            }
        }
    }
    dp[n]
}

#[test]
fn test() {
    let cases = vec![
        ("leetcode", vec!["leet", "code"], true),
        ("applepenapple", vec!["apple", "pen"], true),
        (
            "catsandog",
            vec!["cats", "dog", "sand", "and", "cat"],
            false,
        ),
    ];
    for (word, dict, expected) in cases {
        assert_eq!(word_break(word.into(), utils::vec_of(dict)), expected);
    }
}
