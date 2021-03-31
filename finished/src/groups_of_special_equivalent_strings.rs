use std::collections::HashSet;

pub fn num_special_equiv_groups(arr: Vec<String>) -> i32 {
    let mut set = HashSet::new();
    for s in arr {
        let word = Word::from_string(&s);
        set.insert(word);
    }
    set.len() as i32
}

#[derive(Hash, Eq, PartialEq)]
struct Word {
    odds: [u8; 26],
    evens: [u8; 26],
}

impl Word {
    fn from_string(s: &str) -> Word {
        let mut odds = [0u8; 26];
        let mut evens = [0u8; 26];
        for (i, ch) in s.bytes().enumerate() {
            let idx = (ch - b'a') as usize;
            if i & 1 == 0 {
                evens[idx] += 1;
            } else {
                odds[idx] += 1;
            }
        }
        Word { odds, evens }
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec!["a", "b", "c", "a", "c", "c"], 3),
        (vec!["aa", "bb", "ab", "ba"], 4),
        (vec!["abc", "acb", "bac", "bca", "cab", "cba"], 3),
        (vec!["abcd", "cdab", "adcb", "cbad"], 1),
    ];
    for (input, expected) in cases {
        let input: Vec<_> = input.into_iter().map(|s| s.to_string()).collect();
        assert_eq!(num_special_equiv_groups(input), expected);
    }
}
