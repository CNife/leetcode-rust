use std::cmp::max;
use std::collections::BTreeMap;

pub fn find_lhs(nums: Vec<i32>) -> i32 {
    let mut num_counts = BTreeMap::new();
    for num in nums {
        *num_counts.entry(num).or_insert(0) += 1;
    }

    let mut prev_num = -1;
    let mut prev_count = -1;
    let mut max_count_sum = 0;
    for (num, count) in num_counts {
        if prev_count > 0 && prev_num + 1 == num {
            let count_sum = count + prev_count;
            max_count_sum = max(max_count_sum, count_sum);
        }
        prev_num = num;
        prev_count = count;
    }

    max_count_sum
}

#[test]
fn test() {
    let cases = vec![(vec![1, 3, 2, 2, 5, 2, 3, 7], 5)];
    for (nums, expected) in cases {
        assert_eq!(find_lhs(nums), expected);
    }
}
