use std::cmp::max;

pub fn rob(nums: Vec<i32>) -> i32 {
    fn simple_rob(slice: &[i32]) -> i32 {
        let mut prev = 0;
        let mut curr = 0;
        for &num in slice {
            let old_curr = curr;
            curr = max(curr, prev + num);
            prev = old_curr;
        }
        curr
    }

    match nums.len() {
        0 => 0,
        1 => nums[0],
        _ => max(simple_rob(&nums[1..]), simple_rob(&nums[..nums.len() - 1])),
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![], 0),
        (vec![1], 1),
        (vec![2, 3, 2], 3),
        (vec![1, 2, 3, 1], 4),
    ];
    for (nums, expected) in cases {
        assert_eq!(rob(nums), expected);
    }
}
