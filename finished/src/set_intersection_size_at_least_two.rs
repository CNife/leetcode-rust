pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_unstable_by(|lhs, rhs| lhs[0].cmp(&rhs[0]).then(rhs[1].cmp(&lhs[1])));
    let n = intervals.len();
    let mut cur = intervals[n - 1][0];
    let mut next = cur + 1;
    let mut result = 2;
    for interval in intervals[0..n - 1].iter().rev() {
        if interval[1] >= next {
            continue;
        } else if interval[1] < cur {
            cur = interval[0];
            next = interval[0] + 1;
            result += 2;
        } else {
            next = cur;
            cur = interval[0];
            result += 1;
        }
    }
    result
}

#[test]
fn test_intersection_size_two() {
    let tests = vec![
        (vec![vec![1, 3], vec![1, 4], vec![2, 5], vec![3, 5]], 3),
        (vec![vec![1, 2], vec![2, 3], vec![2, 4], vec![4, 5]], 5),
    ];
    for (intervals, want) in tests {
        assert_eq!(intersection_size_two(intervals), want);
    }
}
