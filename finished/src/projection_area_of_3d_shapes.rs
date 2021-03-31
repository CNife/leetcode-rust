pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
    let rows: i32 = grid.iter().map(|row| *row.iter().max().unwrap()).sum();
    let columns: i32 = (0..grid.len())
        .map(|c| grid.iter().map(|row| row[c]).max().unwrap())
        .sum();
    let downs = grid.into_iter().flatten().filter(|h| *h != 0).count();
    rows + columns + downs as i32
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![2]], 5),
        (vec![vec![1, 2], vec![3, 4]], 17),
        (vec![vec![1, 0], vec![0, 2]], 8),
        (vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]], 14),
        (vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]], 21),
    ];
    for (grid, expected) in cases {
        assert_eq!(projection_area(grid), expected);
    }
}
