use std::cmp::max;

pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();

    let n = nums.len();
    let max_3_product = nums[n - 3] * nums[n - 2] * nums[n - 1];
    if nums[0] < 0 && nums[1] < 0 {
        max(max_3_product, nums[0] * nums[1] * nums[n - 1])
    } else {
        max_3_product
    }
}

#[test]
fn test() {
    let cases = vec![(vec![1, 2, 3], 6), (vec![-1, -2, 0, 1], 2)];
    for (nums, expected) in cases {
        assert_eq!(maximum_product(nums), expected);
    }
}
