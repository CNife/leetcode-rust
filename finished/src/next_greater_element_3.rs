pub struct Solution;

impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut digits = n.to_string().into_bytes();
        let (i, ei) = match digits
            .iter()
            .enumerate()
            .zip(digits[1..].iter())
            .rfind(|((_, el), er)| el < er)
        {
            Some((res, _)) => res,
            None => return -1,
        };
        let (j, _) = digits.iter().enumerate().rfind(|&(_, ej)| ej > ei).unwrap();
        digits.swap(i, j);
        digits[i + 1..].reverse();
        unsafe {
            String::from_utf8_unchecked(digits)
                .parse::<i32>()
                .unwrap_or(-1)
        }
    }
}

#[test]
fn test_next_greater_element() {
    let cases = vec![(12, 21), (21, -1)];
    for (n, expected) in cases {
        let output = Solution::next_greater_element(n);
        assert_eq!(output, expected);
    }
}
