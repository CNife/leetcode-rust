pub struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut matrix = Solution::init_matrix(n);

        let mut counter = 0i32;
        let mut next = || {
            counter += 1;
            counter
        };

        let mut i = 0;
        let mut j = n - 1;
        while i < j {
            for c in i..j {
                matrix[i][c] = next();
            }
            for r in i..j {
                matrix[r][j] = next();
            }
            for c in (i + 1..j + 1).rev() {
                matrix[j][c] = next();
            }
            for r in (i + 1..j + 1).rev() {
                matrix[r][i] = next();
            }
            i += 1;
            j -= 1;
        }
        if i == j {
            matrix[i][i] = next();
        }
        matrix
    }

    fn init_matrix(n: usize) -> Vec<Vec<i32>> {
        let mut matrix = Vec::with_capacity(n);
        for _ in 0..n {
            let row = vec![0; n];
            matrix.push(row);
        }
        matrix
    }
}

#[test]
fn test_generate_matrix() {
    let cases = vec![(3, vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]])];
    for (n, expected) in cases {
        assert_eq!(Solution::generate_matrix(n), expected);
    }
}
