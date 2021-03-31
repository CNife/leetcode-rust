use std::collections::{HashMap, HashSet};

pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut cnt_map = HashMap::new();
    for num in arr {
        let cnt = cnt_map.entry(num).or_insert(0);
        *cnt += 1;
    }

    let mut cnt_set = HashSet::with_capacity(cnt_map.len());
    cnt_map.values().all(|cnt| cnt_set.insert(*cnt))
}

#[test]
fn test_unique_occurrences() {
    let cases = vec![
        (vec![1, 2, 2, 1, 1, 3], true),
        (vec![1, 2], false),
        (vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0], true),
    ];
    for (arr, expected) in cases {
        let output = unique_occurrences(arr);
        assert_eq!(output, expected);
    }
}
