use std::cmp::max;

pub fn rob(nums: Vec<i32>) -> i32 {
    let mut a = 0;
    let mut b = 0;

    for (i, &num) in nums.iter().enumerate() {
        if i % 2 == 0 {
            a = max(a + num, b);
        } else {
            b = max(a, b + num);
        }
    }
    max(a, b)
}

#[test]
fn test() {
    let cases = vec![(vec![1, 2, 3, 1], 4), (vec![2, 7, 9, 3, 1], 12)];
    for (nums, expect) in cases {
        assert_eq!(rob(nums), expect);
    }
}
