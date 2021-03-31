pub struct Solution;

use std::cmp::{max, min};

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = height.len() - 1;
        let mut res = 0;
        while i < j {
            res = max(res, min(height[i], height[j]) * (j - i) as i32);
            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }
        res
    }
}

#[test]
fn test_max_area() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
}
