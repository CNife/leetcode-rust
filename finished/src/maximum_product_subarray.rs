use std::cmp::{max, min};
use std::i32;
use std::mem::swap;

pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut result = i32::MIN;
    let mut imax = 1;
    let mut imin = 1;

    for num in nums {
        if num < 0 {
            swap(&mut imax, &mut imin);
        }
        imax = max(imax * num, num);
        imin = min(imin * num, num);
        result = max(result, imax);
    }
    result
}

#[test]
fn test() {
    let cases = vec![(vec![2, 3, -2, 4], 6), (vec![-2, 0, -1], 0)];
    for (nums, expected) in cases {
        assert_eq!(max_product(nums), expected);
    }
}
