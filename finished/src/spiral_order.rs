pub struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        if matrix.is_empty() {
            return result;
        }

        let mut row_start = 0;
        let mut row_end = matrix.len();
        let mut column_start = 0;
        let mut column_end = matrix[0].len();
        while row_end - row_start >= 2 && column_end - column_start >= 2 {
            for c in column_start..column_end {
                result.push(matrix[row_start][c]);
            }
            for r in row_start + 1..row_end - 1 {
                result.push(matrix[r][column_end - 1]);
            }
            for c in (column_start..column_end).rev() {
                result.push(matrix[row_end - 1][c]);
            }
            for r in (row_start + 1..row_end - 1).rev() {
                result.push(matrix[r][column_start]);
            }

            row_start += 1;
            row_end -= 1;
            column_start += 1;
            column_end -= 1;
        }

        if row_end - row_start == 1 {
            for c in column_start..column_end {
                result.push(matrix[row_start][c]);
            }
        } else if column_end - column_start == 1 {
            for r in row_start..row_end {
                result.push(matrix[r][column_start]);
            }
        }

        result
    }
}

#[test]
fn test_spiral_order() {
    let cases = vec![
        (
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
        ),
        (
            vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]],
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
        ),
        (vec![vec![3], vec![2]], vec![3, 2]),
        (
            vec![
                vec![1, 11],
                vec![2, 12],
                vec![3, 13],
                vec![4, 14],
                vec![5, 15],
                vec![6, 16],
                vec![7, 17],
                vec![8, 18],
                vec![9, 19],
                vec![10, 20],
            ],
            vec![
                1, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 10, 9, 8, 7, 6, 5, 4, 3, 2,
            ],
        ),
    ];
    for (input, expected) in cases {
        assert_eq!(Solution::spiral_order(input), expected);
    }
}
