pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temps: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; temps.len()];
        let mut stack: Vec<(usize, i32)> = vec![];
        for (i, elem) in temps.iter().enumerate().rev() {
            loop {
                match stack.last() {
                    Some((_, t)) if t <= elem => stack.pop(),
                    _ => break,
                };
            }
            result[i] = stack.last().map(|(j, _)| *j - i).unwrap_or(0) as i32;
            stack.push((i, *elem));
        }
        result
    }
}

#[test]
fn test_daily_temperatures() {
    let input = vec![73, 74, 75, 71, 69, 72, 76, 73];
    let expected = vec![1, 1, 4, 2, 1, 1, 0, 0];
    let output = Solution::daily_temperatures(input);
    assert_eq!(output, expected);
}
