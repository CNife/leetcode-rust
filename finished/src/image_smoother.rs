pub fn image_smoother(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if matrix.is_empty() {
        return vec![];
    }
    let m = matrix.len();
    let n = matrix[0].len();

    let mut result = vec![vec![0; n]; m];
    for x in 0..m {
        for y in 0..n {
            let mut counter = 0;
            let left = x.wrapping_sub(1);
            let right = x + 1;
            let down = y.wrapping_sub(1);
            let up = y + 1;

            macro_rules! cell {
                ($x: expr, $y: expr) => {
                    if $x < m && $y < n {
                        counter += 1;
                        matrix[$x][$y]
                    } else {
                        0
                    }
                };
            }

            #[allow(clippy::eval_order_dependence)]
            let sum = cell!(left, up)
                + cell!(x, up)
                + cell!(right, up)
                + cell!(left, y)
                + cell!(x, y)
                + cell!(right, y)
                + cell!(left, down)
                + cell!(x, down)
                + cell!(right, down);
            result[x][y] = sum / counter;
        }
    }
    result
}

#[test]
fn test() {
    let cases = vec![(
        vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]],
        vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
    )];
    for (matrix, expect) in cases {
        assert_eq!(image_smoother(matrix), expect);
    }
}
