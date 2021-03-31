use std::collections::HashSet;

pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
    if k < 0 {
        0
    } else {
        let mut num_set = HashSet::new();
        let mut diff_set = HashSet::new();
        for n in nums {
            if num_set.contains(&(n - k)) {
                diff_set.insert(n - k);
            }
            if num_set.contains(&(n + k)) {
                diff_set.insert(n);
            }
            num_set.insert(n);
        }
        diff_set.len() as i32
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![3, 1, 4, 1, 5], 2, 2),
        (vec![1, 2, 3, 4, 5], 1, 4),
        (vec![1, 3, 1, 5, 4], 0, 1),
    ];
    for (nums, k, expected) in cases {
        assert_eq!(find_pairs(nums, k), expected);
    }
}
