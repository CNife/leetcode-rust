pub struct Solution;

impl Solution {
    pub fn find_occurrence(text: String, first: String, second: String) -> Vec<String> {
        let words: Vec<_> = text.split(' ').collect();
        let mut result = vec![];

        for i in 0..words.len() - 2 {
            if words[i] == first && words[i + 1] == second {
                result.push(words[i + 2].to_string());
            }
        }
        result
    }
}

#[test]
fn test_find_occurrence() {
    let cases = vec![
        (
            "alice is a good girl she is a good student",
            "a",
            "good",
            vec!["girl", "student"],
        ),
        ("we will we will rock you", "we", "will", vec!["we", "rock"]),
    ];
    for case in cases {
        let result =
            Solution::find_occurrence(case.0.to_string(), case.1.to_string(), case.2.to_string());
        let expect: Vec<_> = case.3.into_iter().map(|s| s.to_string()).collect();
        assert_eq!(result, expect);
    }
}
