#[allow(unused_imports)]
use Direction::*;

#[allow(dead_code)]
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Direction {
    Start,
    Left,
    Right,
    Up,
    Down,
}

struct Context {
    board: Vec<Vec<char>>,
    visited: Vec<Vec<bool>>,
    m: usize,
    n: usize,
}

impl Context {
    fn new(board: Vec<Vec<char>>) -> Context {
        let m = board.len();
        let n = board[0].len();
        let visited = vec![vec![false; n]; m];
        Context {
            board,
            visited,
            m,
            n,
        }
    }
}

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    if word.is_empty() {
        return true;
    }

    let mut ctx = Context::new(board);
    let first = word.chars().next().unwrap();
    for i in 0..ctx.m {
        for j in 0..ctx.n {
            if ctx.board[i][j] == first && backtrack(&mut ctx, i, j, &word[1..]) {
                return true;
            }
        }
    }
    false
}

fn backtrack(ctx: &mut Context, x: usize, y: usize, word: &str) -> bool {
    if word.is_empty() {
        return true;
    }

    ctx.visited[x][y] = true;
    let head = word.chars().next().unwrap();

    macro_rules! probe {
        ($pre : expr, $x: expr, $y:expr) => {
            $pre && ctx.board[$x][$y] == head
                && !ctx.visited[$x][$y]
                && backtrack(ctx, $x, $y, &word[1..])
        };
    }

    let result = probe!(y > 0, x, y - 1)
        || probe!(y < ctx.board[0].len() - 1, x, y + 1)
        || probe!(x > 0, x - 1, y)
        || probe!(x < ctx.board.len() - 1, x + 1, y);
    ctx.visited[x][y] = false;
    result
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            vec![("ABCCED", true), ("SEE", true), ("ABCB", false)],
        ),
        (
            vec![
                vec!['a', 'a', 'a', 'a'],
                vec!['a', 'a', 'a', 'a'],
                vec!['a', 'a', 'a', 'a'],
            ],
            vec![("aaaaaaaaaaaaa", false)],
        ),
    ];
    for (board, words) in &cases {
        for (word, expected) in words {
            assert_eq!(exist(board.clone(), word.to_string()), *expected);
        }
    }
}
