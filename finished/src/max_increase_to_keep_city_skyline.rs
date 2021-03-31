use std::cmp::min;

pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
    grid_sum(&max_grid(row_skyline(&grid), column_skyline(&grid))) - grid_sum(&grid)
}

fn row_skyline(grid: &[Vec<i32>]) -> Vec<i32> {
    grid.iter().map(|row| *row.iter().max().unwrap()).collect()
}

fn column_skyline(grid: &[Vec<i32>]) -> Vec<i32> {
    (0..grid[0].len())
        .map(|c| (0..grid.len()).map(|r| grid[r][c]).max().unwrap())
        .collect()
}

fn grid_sum(grid: &[Vec<i32>]) -> i32 {
    grid.iter().map(|row| row.iter().sum::<i32>()).sum()
}

fn max_grid(row_skyline: Vec<i32>, column_skyline: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![vec![0; column_skyline.len()]; row_skyline.len()];
    for x in 0..row_skyline.len() {
        for y in 0..column_skyline.len() {
            result[x][y] = min(row_skyline[x], column_skyline[y]);
        }
    }
    result
}

#[test]
fn test() {
    let cases = vec![(
        vec![
            vec![3, 0, 8, 4],
            vec![2, 4, 5, 7],
            vec![9, 2, 6, 3],
            vec![0, 3, 1, 0],
        ],
        35,
    )];
    for (grid, expected) in cases {
        assert_eq!(max_increase_keeping_skyline(grid), expected);
    }
}
