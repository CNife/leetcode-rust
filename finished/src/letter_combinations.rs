pub struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut res = Vec::new();
        Solution::do_combination(digits.as_bytes(), String::new(), &mut res);
        res
    }

    fn do_combination(digits: &[u8], string: String, res: &mut Vec<String>) {
        match digits.get(0) {
            Some(l) => {
                let idx = *l - b'0';
                for next_letter in LETTER_TABLE[idx as usize].chars() {
                    let mut cloned_string = string.clone();
                    cloned_string.push(next_letter);
                    Solution::do_combination(&digits[1..], cloned_string, res);
                }
            }
            None => {
                if !string.is_empty() {
                    res.push(string);
                }
            }
        }
    }
}

const LETTER_TABLE: [&str; 10] = [
    "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
];

#[test]
fn test_letter_combinations() {
    use std::collections::HashSet;

    let input = "23".to_string();
    let expected = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<HashSet<_>>();
    assert_eq!(
        Solution::letter_combinations(input)
            .into_iter()
            .collect::<HashSet<_>>(),
        expected
    );
}
