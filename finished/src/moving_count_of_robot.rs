pub fn moving_count(m: i32, n: i32, k: i32) -> i32 {
    let mut ctx = Context::new(m as usize, n as usize, k as usize);
    ctx.backtrack(0, 0);
    ctx.valid_count
}

struct Context {
    visited: Vec<Vec<bool>>,
    valid_count: i32,
    m: usize,
    n: usize,
    k: usize,
}

impl Context {
    fn new(m: usize, n: usize, k: usize) -> Context {
        Context {
            visited: vec![vec![false; n]; m],
            valid_count: 0,
            m,
            n,
            k,
        }
    }

    fn backtrack(&mut self, x: usize, y: usize) {
        debug_assert!(x < self.m);
        debug_assert!(y < self.n);

        debug_assert!(!self.visited[x][y]);
        self.visited[x][y] = true;

        if !self.is_valid(x, y) {
            return;
        }

        self.valid_count += 1;

        macro_rules! next {
            ($predicate: expr, $x: expr, $y: expr) => {
                if $predicate && !self.visited[$x][$y] {
                    self.backtrack($x, $y);
                }
            };
        }

        let left = y.wrapping_sub(1) < self.n;
        let right = y + 1 < self.n;
        let up = x.wrapping_sub(1) < self.m;
        let down = x + 1 < self.m;
        next!(left && up, x - 1, y - 1);
        next!(up, x - 1, y);
        next!(right && up, x - 1, y + 1);
        next!(left, x, y - 1);
        next!(right, x, y + 1);
        next!(left && down, x + 1, y - 1);
        next!(down, x + 1, y);
        next!(right && down, x + 1, y + 1);
    }

    fn is_valid(&self, x: usize, y: usize) -> bool {
        debug_assert!(x < self.m);
        debug_assert!(y < self.n);

        fn digit_sum(mut num: usize) -> usize {
            let mut result = 0;
            while num != 0 {
                result += num % 10;
                num /= 10;
            }
            result
        }
        digit_sum(x) + digit_sum(y) <= self.k
    }
}

#[test]
fn test() {
    let cases = vec![(2, 3, 1, 3), (3, 1, 0, 1)];
    for (m, n, k, expect) in cases {
        assert_eq!(moving_count(m, n, k), expect);
    }
}
