use std::collections::HashSet;

#[derive(Debug)]
struct Context {
    input: Vec<u8>,
    result: HashSet<Vec<u8>>,
    path: Vec<u8>,
    max_path_len: usize,
}

pub fn remove_invalid_parentheses(input: String) -> Vec<String> {
    let mut ctx = Context {
        input: input.into_bytes(),
        result: HashSet::new(),
        path: Vec::new(),
        max_path_len: 0,
    };
    backtrack(&mut ctx, 0, 0, 0);
    ctx.result
        .into_iter()
        .map(|bytes| unsafe { String::from_utf8_unchecked(bytes) })
        .collect()
}

fn backtrack(ctx: &mut Context, index: usize, left: usize, right: usize) {
    if index >= ctx.input.len() {
        if left == right && ctx.path.len() >= ctx.max_path_len {
            ctx.max_path_len = ctx.path.len();
            ctx.result.insert(ctx.path.clone());
        }
    } else {
        let mut new_index = index;
        while new_index < ctx.input.len() {
            let ch = ctx.input[new_index];
            if ch != b'(' && ch != b')' {
                ctx.path.push(ch);
                new_index += 1;
            } else {
                break;
            }
        }

        if new_index < ctx.input.len() {
            let ch = ctx.input[new_index];
            ctx.path.push(ch);
            if ch == b'(' {
                backtrack(ctx, new_index + 1, left + 1, right);
            } else if left > right {
                backtrack(ctx, new_index + 1, left, right + 1);
            }
            ctx.path.pop();
        }

        backtrack(ctx, new_index + 1, left, right);
        while let Some(&ch) = ctx.path.last() {
            if ch != b'(' && ch != b')' {
                ctx.path.pop();
            } else {
                break;
            }
        }
    }
}

#[test]
fn test() {
    use utils::assert_same_set;

    let cases = vec![
        ("()())()", vec!["()()()", "(())()"]),
        ("(a)())()", vec!["(a)()()", "(a())()"]),
        (")(", vec![""]),
        ("n", vec!["n"]),
        (")(f", vec!["f"]),
    ];
    for (input, expected) in cases {
        assert_same_set(remove_invalid_parentheses(input.to_string()), expected);
    }
}
