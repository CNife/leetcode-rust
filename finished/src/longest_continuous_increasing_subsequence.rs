use std::cmp::max;

pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut prev = nums[0];
    let mut len = 1;
    let mut max_len = 1;
    for &num in &nums[1..] {
        if num > prev && len > 0 {
            len += 1;
        } else {
            len = 1;
        }
        max_len = max(max_len, len);
        prev = num;
    }
    max_len
}

#[test]
fn test() {
    let cases = vec![(vec![1, 3, 5, 4, 7], 3), (vec![2, 2, 2, 2, 2], 1)];
    for (nums, expect) in cases {
        assert_eq!(find_length_of_lcis(nums), expect);
    }
}
