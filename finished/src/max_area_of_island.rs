use std::cmp::max;

pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut max_area = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 1 {
                max_area = max(max_area, dfs(&mut grid, i, j));
            }
        }
    }
    max_area
}

fn dfs(grid: &mut Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    if x >= grid.len() || y >= grid[0].len() || grid[x][y] == 0 {
        0
    } else {
        grid[x][y] = 0;
        1 + dfs(grid, x.wrapping_sub(1), y)
            + dfs(grid, x, y.wrapping_sub(1))
            + dfs(grid, x + 1, y)
            + dfs(grid, x, y + 1)
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
            ],
            6,
        ),
        (
            vec![
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 1],
                vec![0, 0, 0, 1, 1],
            ],
            4,
        ),
    ];
    for (grid, expected) in cases {
        assert_eq!(max_area_of_island(grid), expected);
    }
}
