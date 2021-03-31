pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let m = obstacle_grid.len();
    let n = obstacle_grid[0].len();

    let mut dp = vec![0; n];
    dp[0] = 1;
    for i in 0..m {
        for j in 0..n {
            dp[j] = match obstacle_grid[i][j] {
                0 => dp[j] + if j == 0 { 0 } else { dp[j - 1] },
                1 => 0,
                _ => unreachable!(),
            }
        }
    }
    dp[n - 1]
}

#[test]
fn test() {
    let cases = vec![(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]], 2)];
    for (grid, expected) in cases {
        assert_eq!(unique_paths_with_obstacles(grid), expected);
    }
}
