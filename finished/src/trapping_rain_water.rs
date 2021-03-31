use std::cmp::{max, min};

pub fn trap(heights: Vec<i32>) -> i32 {
    let left_maxes: Vec<i32> = heights
        .iter()
        .scan(0, |left_max, val| {
            *left_max = max(*left_max, *val);
            Some(*left_max)
        })
        .collect();
    heights
        .iter()
        .zip(left_maxes.into_iter())
        .rev()
        .scan(0, |right_max, (&height, left_max)| {
            *right_max = max(*right_max, height);
            Some(min(left_max, *right_max) - height)
        })
        .sum::<i32>()
}

#[test]
fn test() {
    let cases = vec![(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], 6)];
    for (heights, expected) in cases {
        assert_eq!(trap(heights), expected);
    }
}
