use std::collections::VecDeque;

const UNDISCOVERED_LAND: char = '1';
const DISCOVERED_LAND: char = 'D';

pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    if grid.is_empty() {
        return 0;
    }

    let mut result = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == UNDISCOVERED_LAND {
                result += 1;
                bfs(&mut grid, i, j);
            }
        }
    }
    result
}

fn bfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
    let m = grid.len();
    let n = grid[0].len();
    let mut queue = VecDeque::new();
    queue.push_back((i, j));

    while let Some((x, y)) = queue.pop_front() {
        grid[x][y] = DISCOVERED_LAND;
        if x > 0 && grid[x - 1][y] == UNDISCOVERED_LAND {
            grid[x - 1][y] = DISCOVERED_LAND;
            queue.push_back((x - 1, y));
        }
        if y > 0 && grid[x][y - 1] == UNDISCOVERED_LAND {
            grid[x][y - 1] = DISCOVERED_LAND;
            queue.push_back((x, y - 1));
        }
        if x < m - 1 && grid[x + 1][y] == UNDISCOVERED_LAND {
            grid[x + 1][y] = DISCOVERED_LAND;
            queue.push_back((x + 1, y));
        }
        if y < n - 1 && grid[x][y + 1] == UNDISCOVERED_LAND {
            grid[x][y + 1] = DISCOVERED_LAND;
            queue.push_back((x, y + 1));
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0'],
            ],
            1,
        ),
        (
            vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1'],
            ],
            3,
        ),
    ];
    for (grid, expected) in cases {
        assert_eq!(num_islands(grid), expected);
    }
}
