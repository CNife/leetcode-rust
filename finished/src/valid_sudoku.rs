pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    validate_rows(&board) && validate_columns(&board) && validate_grids(&board)
}

fn validate_rows(board: &[Vec<char>]) -> bool {
    board.iter().all(|row| is_valid(row.iter().cloned()))
}

fn validate_columns(board: &[Vec<char>]) -> bool {
    (0..9).all(|i| is_valid((0..9).map(|j| board[j][i])))
}

fn validate_grids(board: &[Vec<char>]) -> bool {
    (0..3).all(|i| {
        (0..3).all(|j| {
            is_valid(GridGenerator { n: 3, i: 0, j: 0 }.map(|(m, n)| board[i * 3 + m][j * 3 + n]))
        })
    })
}

fn is_valid<I: Iterator<Item = char>>(iter: I) -> bool {
    let mut marks = [false; 9];
    for ch in iter {
        if let '1'..='9' = ch {
            let index = (ch as u8 - b'1') as usize;
            if marks[index] {
                return false;
            } else {
                marks[index] = true;
            }
        }
    }
    true
}

struct GridGenerator {
    n: usize,
    i: usize,
    j: usize,
}

impl Iterator for GridGenerator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.j >= self.n {
            self.j = 0;
            self.i += 1;
        }
        if self.i >= self.n {
            None
        } else {
            self.j += 1;
            Some((self.i, self.j - 1))
        }
    }
}

#[test]
fn test_grid_generator() {
    let mut expected = vec![];
    for i in 0..3usize {
        for j in 0..3usize {
            expected.push((i, j));
        }
    }
    assert_eq!(
        GridGenerator { n: 3, i: 0, j: 0 }.collect::<Vec<_>>(),
        expected
    );
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ],
            true,
        ),
        (
            vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ],
            false,
        ),
        (
            vec![
                vec!['.', '.', '4', '.', '.', '.', '6', '3', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['5', '.', '.', '.', '.', '.', '.', '9', '.'],
                vec!['.', '.', '.', '5', '6', '.', '.', '.', '.'],
                vec!['4', '.', '3', '.', '.', '.', '.', '.', '1'],
                vec!['.', '.', '.', '7', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '5', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            ],
            false,
        ),
    ];
    for (board, expected) in cases {
        //        assert_eq!(validate_columns(&board), false);
        assert_eq!(is_valid_sudoku(board), expected);
    }
}
