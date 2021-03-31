macro_rules! fuck {
    ($board: expr, $predicate: expr, $x: expr, $y: expr) => {
        if $predicate && $board[$x][$y] == 'O' {
            mark_not_surrounded($board, $x, $y);
        }
    };
    ($board: expr, $x: expr, $y: expr) => {
        fuck!($board, true, $x, $y)
    };
}

pub fn solve(board: &mut Vec<Vec<char>>) {
    let m = board.len();
    if m == 0 {
        return;
    }
    let n = board[0].len();

    for x in 0..m {
        fuck!(board, x, 0);
        fuck!(board, x, n - 1);
    }
    for y in 1..n - 1 {
        fuck!(board, 0, y);
        fuck!(board, m - 1, y);
    }

    for row in board.iter_mut() {
        for cell in row.iter_mut() {
            *cell = match *cell {
                'o' => 'O',
                'X' | 'O' => 'X',
                _ => unreachable!(),
            };
        }
    }
}

fn mark_not_surrounded(board: &mut Vec<Vec<char>>, x: usize, y: usize) {
    board[x][y] = 'o';

    fuck!(board, x > 0, x - 1, y);
    fuck!(board, x < board.len() - 1, x + 1, y);
    fuck!(board, y > 0, x, y - 1);
    fuck!(board, y < board[0].len() - 1, x, y + 1);
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'O', 'X'],
                vec!['X', 'X', 'O', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ],
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ],
        ),
        (
            vec![
                vec!['O', 'X', 'X', 'O', 'X'],
                vec!['X', 'O', 'O', 'X', 'O'],
                vec!['X', 'O', 'X', 'O', 'X'],
                vec!['O', 'X', 'O', 'O', 'O'],
                vec!['X', 'X', 'O', 'X', 'O'],
            ],
            vec![
                vec!['O', 'X', 'X', 'O', 'X'],
                vec!['X', 'X', 'X', 'X', 'O'],
                vec!['X', 'X', 'X', 'O', 'X'],
                vec!['O', 'X', 'O', 'O', 'O'],
                vec!['X', 'X', 'O', 'X', 'O'],
            ],
        ),
        (vec![], vec![]),
    ];
    for (mut board, expected) in cases {
        solve(&mut board);
        assert_eq!(board, expected);
    }
}
