use std::cmp::max;

pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
    let mut pos = 0;
    let mut pos_left_max = nums[0];
    let mut current_max = nums[0];
    for (i, &num) in nums.iter().enumerate().skip(1) {
        current_max = max(current_max, num);
        if num < pos_left_max {
            pos = i;
            pos_left_max = current_max;
        }
    }
    pos as i32 + 1
}

#[test]
fn test() {
    let cases = vec![(vec![5, 0, 3, 8, 6], 3), (vec![1, 1, 1, 0, 6, 12], 4)];
    for (nums, expected) in cases {
        assert_eq!(partition_disjoint(nums), expected);
    }
}
