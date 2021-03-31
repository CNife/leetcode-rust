use std::cmp::max;

pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let max_sum = find_max_sum(&nums, k as usize);
    max_sum as f64 / k as f64
}

fn find_max_sum(nums: &[i32], k: usize) -> i32 {
    if nums.len() <= k {
        nums.iter().cloned().sum()
    } else {
        let mut sum: i32 = nums[..k].iter().sum();
        let mut result = sum;
        for i in k..nums.len() {
            sum += nums[i];
            sum -= nums[i - k];
            result = max(result, sum);
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![(vec![1, 12, -5, -6, 50, 3], 4, 12.75)];
    for (nums, k, expected) in cases {
        let max_diff = 1.0 / k as f64;
        let output = find_max_average(nums, k);
        assert!((output - expected).abs() < max_diff);
    }
}
