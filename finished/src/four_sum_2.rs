use std::collections::HashMap;

pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for a_num in a {
        for b_num in &b {
            *map.entry(a_num + *b_num).or_insert(0) += 1;
        }
    }

    let mut result = 0;
    for c_num in c {
        for d_num in &d {
            result += map.get(&(-c_num - *d_num)).map_or(0, |count| *count);
        }
    }

    result
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2], 2),
        (
            vec![-1, 1, 1, 1, -1],
            vec![0, -1, -1, 0, 1],
            vec![-1, -1, 1, -1, -1],
            vec![0, 1, 0, -1, -1],
            132,
        ),
    ];
    for (a, b, c, d, expected) in cases {
        assert_eq!(four_sum_count(a, b, c, d), expected);
    }
}
