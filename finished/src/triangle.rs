use std::cmp::min;

pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let mut dp = vec![0; triangle.len()];
    for row in triangle {
        for (i, &num) in row.iter().enumerate().rev() {
            let prev_min = if i == 0 {
                dp[0]
            } else if i == row.len() - 1 {
                dp[i - 1]
            } else {
                min(dp[i - 1], dp[i])
            };
            dp[i] = prev_min + num;
        }
    }
    dp.into_iter().min().unwrap_or(0)
}

#[test]
fn test() {
    let cases = vec![
        (vec![], 0),
        (
            vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]],
            11,
        ),
        (
            vec![
                vec![7],
                vec![-5, 9],
                vec![6, 5, 2],
                vec![-8, -2, -7, 3],
                vec![-2, 6, -6, -1, 4],
            ],
            -6,
        ),
    ];
    for (triangle, expected) in cases {
        assert_eq!(minimum_total(triangle), expected);
    }
}
