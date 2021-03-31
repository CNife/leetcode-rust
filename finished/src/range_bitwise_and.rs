pub struct Solution;

impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        for i in 0..32 {
            if (m >> i) == (n >> i) {
                return m & (-1 << i);
            }
        }
        unreachable!();
    }
}

#[test]
fn test_range_bitwise_and() {
    let cases = vec![(5, 7, 4), (0, 1, 0)];
    for (m, n, expected) in cases {
        assert_eq!(Solution::range_bitwise_and(m, n), expected);
    }
}
