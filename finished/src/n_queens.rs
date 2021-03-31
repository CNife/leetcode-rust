pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut ctx = Context::init(n);
    ctx.backtrace(0);
    ctx.results
}

struct Context {
    n: usize,
    queens: Vec<(usize, usize)>,
    columns: Vec<bool>,
    diagonals: Vec<bool>,
    rev_diagonals: Vec<bool>,
    results: Vec<Vec<String>>,
}

impl Context {
    fn init(n: i32) -> Context {
        let n = n as usize;
        Context {
            n,
            queens: vec![],
            columns: vec![false; n],
            diagonals: vec![false; n * 2 - 1],
            rev_diagonals: vec![false; n * 2 - 1],
            results: vec![],
        }
    }

    fn backtrace(&mut self, r: usize) {
        for c in 0..self.n {
            if self.could_place(r, c) {
                self.place_queen(r, c);
                if r + 1 == self.n {
                    self.add_result();
                } else {
                    self.backtrace(r + 1);
                }
                self.remove_queen(r, c);
            }
        }
    }

    fn could_place(&mut self, r: usize, c: usize) -> bool {
        !(*self.column(c) || *self.diagonal(r, c) || *self.rev_diagonal(r, c))
    }

    fn place_queen(&mut self, r: usize, c: usize) {
        self.queens.push((r, c));
        *self.column(c) = true;
        *self.diagonal(r, c) = true;
        *self.rev_diagonal(r, c) = true;
    }
    fn remove_queen(&mut self, r: usize, c: usize) {
        self.queens.pop().unwrap();
        *self.column(c) = false;
        *self.diagonal(r, c) = false;
        *self.rev_diagonal(r, c) = false;
    }

    fn add_result(&mut self) {
        let mut result = Vec::with_capacity(self.n);
        for _ in 0..self.n {
            result.push(vec![b'.'; self.n]);
        }
        for &(r, c) in self.queens.iter() {
            result[r][c] = b'Q';
        }
        unsafe {
            self.results.push(
                result
                    .into_iter()
                    .map(|s| String::from_utf8_unchecked(s))
                    .collect(),
            )
        }
    }

    #[inline]
    fn column(&mut self, c: usize) -> &mut bool {
        &mut self.columns[c]
    }
    #[inline]
    fn diagonal(&mut self, r: usize, c: usize) -> &mut bool {
        &mut self.diagonals[r + c]
    }
    #[inline]
    fn rev_diagonal(&mut self, r: usize, c: usize) -> &mut bool {
        &mut self.rev_diagonals[if r >= c { r - c } else { c - r + self.n - 1 }]
    }
}

#[test]
fn test() {
    solve_n_queens(4);
}
