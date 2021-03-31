pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort_unstable_by_key(|interval| (interval[0], interval[1]));

    let mut stack: Vec<Vec<i32>> = Vec::new();
    for interval in intervals {
        match stack.last_mut() {
            Some(left) if left[1] >= interval[0] => {
                left[1] = std::cmp::max(left[1], interval[1]);
            }
            _ => stack.push(interval),
        }
    }
    stack
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]],
            vec![vec![1, 6], vec![8, 10], vec![15, 18]],
        ),
        (vec![vec![1, 4], vec![4, 5]], vec![vec![1, 5]]),
    ];

    for (intervals, expected) in cases {
        assert_eq!(merge(intervals), expected);
    }
}
