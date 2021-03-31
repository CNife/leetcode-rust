use std::collections::HashMap;

pub fn uncommon_from_sentences(a: String, b: String) -> Vec<String> {
    let a_map = count_map(&a);
    let b_map = count_map(&b);
    let mut res = vec![];

    let mut fuck = |src_map: &HashMap<&str, i32>, find_map: &HashMap<&str, i32>| {
        for (word, cnt) in src_map {
            if *cnt == 1 && !find_map.contains_key(word) {
                res.push(word.to_string());
            }
        }
    };

    fuck(&a_map, &b_map);
    fuck(&b_map, &a_map);
    res
}

fn count_map(s: &str) -> HashMap<&str, i32> {
    let mut cnt_map = HashMap::new();
    for word in s.split_ascii_whitespace() {
        let cnt = cnt_map.entry(word).or_insert(0);
        *cnt += 1;
    }
    cnt_map
}

#[test]
fn test() {
    use std::collections::HashSet;

    let cases = vec![
        (
            "this apple is sweet",
            "this apple is sour",
            vec!["sweet", "sour"],
        ),
        ("apple apple", "banana", vec!["banana"]),
    ];
    for (a, b, expected) in cases {
        let output = uncommon_from_sentences(a.to_string(), b.to_string());
        assert_eq!(
            output.into_iter().collect::<HashSet<_>>(),
            expected
                .into_iter()
                .map(|s| s.to_string())
                .collect::<HashSet<_>>()
        );
    }
}
