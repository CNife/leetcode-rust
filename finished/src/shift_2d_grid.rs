pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    let grid_count = m * n;
    let shift_count = (k as usize) % grid_count;

    let mut result = grid.clone();
    for j in (grid_count - shift_count)..grid_count {
        let i = j + shift_count - grid_count;
        result[i / n][i % n] = grid[j / n][j % n];
    }
    for j in 0..(grid_count - shift_count) {
        let i = j + shift_count;
        result[i / n][i % n] = grid[j / n][j % n];
    }
    result
}

#[test]
fn test_shift_grid() {
    let tests = vec![
        (
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            1,
            vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]],
        ),
        (
            vec![
                vec![3, 8, 1, 9],
                vec![19, 7, 2, 5],
                vec![4, 6, 11, 10],
                vec![12, 0, 21, 13],
            ],
            4,
            vec![
                vec![12, 0, 21, 13],
                vec![3, 8, 1, 9],
                vec![19, 7, 2, 5],
                vec![4, 6, 11, 10],
            ],
        ),
        (
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            9,
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        ),
    ];
    for (grid, k, want) in tests {
        assert_eq!(shift_grid(grid, k), want);
    }
}
