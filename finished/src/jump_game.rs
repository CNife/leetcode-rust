use std::cmp::max;

pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut k = 0;
    for i in 0..nums.len() {
        if i > k {
            return false;
        } else if k >= nums.len() - 1 {
            return true;
        }
        k = max(k, i + nums[i] as usize);
    }
    true
}

#[test]
fn test() {
    let cases = vec![(vec![2, 3, 1, 1, 4], true), (vec![3, 2, 1, 0, 4], false)];
    for (nums, expected) in cases {
        assert_eq!(can_jump(nums), expected);
    }
}
