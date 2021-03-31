pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() {
        return false;
    }

    let first_column: Vec<_> = matrix.iter().map(|row| row[0]).collect();
    match first_column.binary_search(&target) {
        Ok(_) => true,
        Err(i) => {
            if i == 0 {
                false
            } else {
                matrix[i - 1].binary_search(&target).is_ok()
            }
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
            3,
            true,
        ),
        (
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
            13,
            false,
        ),
    ];
    for (matrix, target, expected) in cases {
        assert_eq!(search_matrix(matrix, target), expected);
    }
}
