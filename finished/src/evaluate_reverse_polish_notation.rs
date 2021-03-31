pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = Vec::new();
    for token in tokens {
        match token.parse::<i32>() {
            Ok(n) => stack.push(n),
            Err(_) => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                let result = match token.as_bytes()[0] {
                    b'+' => lhs + rhs,
                    b'-' => lhs - rhs,
                    b'*' => lhs * rhs,
                    b'/' => lhs / rhs,
                    _ => unreachable!(),
                };
                stack.push(result);
            }
        }
    }
    stack.pop().unwrap()
}

#[test]
fn test() {
    let cases = vec![
        (vec!["2", "1", "+", "3", "*"], 9),
        (vec!["4", "13", "5", "/", "+"], 6),
        (
            vec![
                "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
            ],
            22,
        ),
    ];
    for (tokens, expected) in cases {
        let tokens: Vec<_> = tokens.into_iter().map(ToString::to_string).collect();
        assert_eq!(eval_rpn(tokens), expected);
    }
}
