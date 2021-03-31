use std::collections::VecDeque;

pub fn update_matrix(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; n]; m];

    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == 0 {
                queue.push_back((i, j));
                visited[i][j] = true;
            }
        }
    }

    let mut distance = 0;
    while !queue.is_empty() {
        let len = queue.len();
        for _ in 0..len {
            let (x, y) = queue.pop_front().unwrap();
            matrix[x][y] = distance;

            macro_rules! next {
                ($predicate: expr, $x: expr, $y: expr) => {
                    if $predicate && !visited[$x][$y] {
                        queue.push_back(($x, $y));
                        visited[$x][$y] = true;
                    }
                };
            }
            next!(x > 0, x - 1, y);
            next!(x < m - 1, x + 1, y);
            next!(y > 0, x, y - 1);
            next!(y < n - 1, x, y + 1);
        }
        distance += 1;
    }
    matrix
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 1, 0]],
            vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 1, 0]],
        ),
        (
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]],
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]],
        ),
    ];
    for (matrix, expect) in cases {
        assert_eq!(update_matrix(matrix), expect);
    }
}
