pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut rows = vec![];
        let mut columns = vec![];

        for (r, row) in matrix.iter().enumerate() {
            for (c, elem) in row.iter().enumerate() {
                if *elem == 0 {
                    rows.push(r);
                    columns.push(c);
                }
            }
        }

        for r in rows {
            for elem in matrix[r].iter_mut() {
                *elem = 0;
            }
        }

        for c in columns {
            for r in 0..matrix.len() {
                matrix[r][c] = 0;
            }
        }
    }
}

#[test]
fn test_set_zeroes() {
    let cases = vec![
        (
            vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]],
            vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]],
        ),
        (
            vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]],
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]],
        ),
    ];
    for (mut matrix, expected) in cases {
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, expected);
    }
}
