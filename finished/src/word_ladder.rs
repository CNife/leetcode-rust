use std::collections::HashSet;
use std::mem::swap;

pub fn ladder_length(begin: String, end: String, words: Vec<String>) -> i32 {
    if !words.contains(&end) {
        return 0;
    }

    let words: HashSet<Vec<u8>> = words.into_iter().map(|word| word.into_bytes()).collect();
    let mut begin_set = HashSet::new();
    begin_set.insert(begin.into_bytes());
    let mut end_set = HashSet::new();
    end_set.insert(end.into_bytes());
    let mut visited = HashSet::new();

    let mut len = 1;
    while !begin_set.is_empty() && !end_set.is_empty() {
        if begin_set.len() > end_set.len() {
            swap(&mut begin_set, &mut end_set);
        }

        let mut tmp_set = HashSet::new();
        for mut word in begin_set {
            for i in 0..word.len() {
                let old_byte = word[i];
                for replace_byte in b'a'..=b'z' {
                    if old_byte == replace_byte {
                        continue;
                    }
                    word[i] = replace_byte;
                    if end_set.contains(&word) {
                        return len + 1;
                    }
                    if !visited.contains(&word) && words.contains(&word) {
                        visited.insert(word.clone());
                        tmp_set.insert(word.clone());
                    }
                }
                word[i] = old_byte;
            }
        }
        begin_set = tmp_set;
        len += 1;
    }
    0
}

#[test]
fn test() {
    let cases = vec![
        (
            "hit",
            "cog",
            vec!["hot", "dot", "dog", "lot", "log", "cog"],
            5,
        ),
        ("hit", "cog", vec!["hot", "dot", "dog", "lot", "log"], 0),
    ];
    for (begin, end, words, expected) in cases {
        assert_eq!(
            ladder_length(begin.into(), end.into(), utils::vec_of(words)),
            expected
        );
    }
}
