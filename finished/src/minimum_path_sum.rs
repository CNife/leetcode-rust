use std::cmp::min;

pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let n = grid[0].len();
    let mut dp = Vec::with_capacity(n);
    for num in &grid[0] {
        dp.push(dp.last().unwrap_or(&0) + num);
    }

    for row in &grid[1..] {
        dp[0] += row[0];
        for i in 1..n {
            dp[i] = min(dp[i], dp[i - 1]) + row[i];
        }
    }
    dp[n - 1]

    //    let mut dp = vec![vec![0; n]; m];
    //
    //    dp[0][0] = grid[0][0];
    //    for i in 1..n {
    //        dp[0][i] = dp[0][i - 1] + grid[0][i];
    //    }
    //
    //    for i in 1..m {
    //        dp[i][0] = dp[i - 1][0] + grid[i][0];
    //        for j in 1..n {
    //            dp[i][j] = min(dp[i - 1][j], dp[i][j - 1]) + grid[i][j];
    //        }
    //    }
    //
    //    dp[m - 1][n - 1]
}

#[test]
fn test() {
    let cases = vec![(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]], 7)];
    for (grid, expected) in cases {
        assert_eq!(min_path_sum(grid), expected);
    }
}
