use std::cmp::max;

struct Context {
    matrix: Vec<Vec<i32>>,
    cache: Vec<Vec<i32>>,
    m: usize,
    n: usize,
}

pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
    let m = matrix.len();
    if m == 0 {
        return 0;
    }
    let n = matrix[0].len();
    let mut ctx = Context {
        matrix,
        cache: vec![vec![0; n]; m],
        m,
        n,
    };

    let mut result = 0;
    for i in 0..m {
        for j in 0..n {
            result = max(result, memorized_dfs(&mut ctx, i, j));
        }
    }
    result
}

fn memorized_dfs(ctx: &mut Context, x: usize, y: usize) -> i32 {
    if ctx.cache[x][y] > 0 {
        return ctx.cache[x][y];
    }

    let this_value = ctx.matrix[x][y];
    let mut result = 0;

    macro_rules! dfs {
        ($guard: expr, $x: expr, $y: expr) => {
            if $guard && ctx.matrix[$x][$y] > this_value {
                result = max(result, memorized_dfs(ctx, $x, $y));
            }
        };
    }
    dfs!(x > 0, x - 1, y);
    dfs!(y > 0, x, y - 1);
    dfs!(x < ctx.m - 1, x + 1, y);
    dfs!(y < ctx.n - 1, x, y + 1);

    result += 1;
    ctx.cache[x][y] = result;
    result
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]], 4),
        (vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]], 4),
    ];
    for (matrix, expected) in cases {
        assert_eq!(longest_increasing_path(matrix), expected);
    }
}
