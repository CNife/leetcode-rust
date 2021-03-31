use std::cmp::max;
use std::collections::VecDeque;

pub fn max_distance(mut grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut queue = VecDeque::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 1 {
                queue.push_back((i, j));
            }
        }
    }
    if queue.len() == n * n {
        return -1;
    }

    let mut curr_depth = -1;
    while let Some((x, y)) = queue.pop_front() {
        macro_rules! push_available {
            ($predicate: expr, $x: expr, $y: expr) => {
                if $predicate && grid[$x][$y] == 0 {
                    grid[$x][$y] = curr_depth + 2;
                    queue.push_back(($x, $y));
                }
            };
        }

        curr_depth = max(curr_depth, grid[x][y] - 1);
        push_available!(x > 0, x - 1, y);
        push_available!(x < n - 1, x + 1, y);
        push_available!(y > 0, x, y - 1);
        push_available!(y < n - 1, x, y + 1);
    }
    curr_depth
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]], 2),
        (vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 0]], 4),
        (vec![vec![0, 0], vec![0, 0]], -1),
        (vec![vec![1, 1], vec![1, 1]], -1),
    ];
    for (grid, expect) in cases {
        assert_eq!(max_distance(grid.clone()), expect, "{:?}", grid);
    }
}
