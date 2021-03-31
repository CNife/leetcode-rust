pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
    let (r, c) = find_rook(&board);
    let row = &board[r];
    judge(row[..c].iter().rev()) // left
        + judge(row[c + 1..].iter()) // right
        + judge((0..r).rev().map(|j| &board[j][c])) // up
        + judge((r + 1..8).map(|j| &board[j][c])) // down
}

fn find_rook(board: &[Vec<char>]) -> (usize, usize) {
    for (r, row) in board.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if *ch == 'R' {
                return (r, c);
            }
        }
    }
    unreachable!()
}

fn judge<'a>(iter: impl Iterator<Item = &'a char>) -> i32 {
    for ch in iter {
        match *ch {
            'p' => return 1,
            '.' => continue,
            _ => break,
        }
    }
    0
}

#[test]
fn test_num_rook_captures() {
    let cases = vec![
        (
            vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'R', '.', '.', '.', 'p'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            ],
            3,
        ),
        (
            vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
                vec!['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
                vec!['.', 'p', 'B', 'R', 'B', 'p', '.', '.'],
                vec!['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
                vec!['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            ],
            0,
        ),
        (
            vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['p', 'p', '.', 'R', '.', 'p', 'B', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            ],
            3,
        ),
    ];

    for (input, expected) in cases {
        let output = num_rook_captures(input);
        assert_eq!(output, expected);
    }
}
