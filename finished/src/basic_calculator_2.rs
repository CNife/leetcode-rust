enum Token {
    Number(i32),
    Operator(char),
}

pub fn calculate(input: String) -> i32 {
    let mut input = input.as_str();
    let mut num_stack = Vec::new();
    let mut op_stack: Vec<char> = Vec::new();

    while let Some((token, next_input)) = next_token(input) {
        match token {
            Token::Number(num) => num_stack.push(num),
            Token::Operator(op) => {
                while !op_stack.is_empty() {
                    let prev_op = op_stack.last().cloned().unwrap();
                    if priority(prev_op) >= priority(op) {
                        op_stack.pop();
                        do_calculation(&mut num_stack, prev_op);
                    } else {
                        break;
                    }
                }
                op_stack.push(op);
            }
        }
        input = next_input;
    }

    while let Some(op) = op_stack.pop() {
        do_calculation(&mut num_stack, op);
    }

    num_stack.pop().unwrap()
}

fn next_token(input: &str) -> Option<(Token, &str)> {
    let input = input.trim_start();
    input.chars().next().map(|first| {
        if first.is_ascii_digit() {
            let num_len = input
                .char_indices()
                .find(|(_, ch)| !ch.is_ascii_digit())
                .map_or(input.len(), |(index, _)| index);
            let num = input[..num_len].parse::<i32>().unwrap();
            (Token::Number(num), &input[num_len..])
        } else {
            (Token::Operator(first), &input[1..])
        }
    })
}

fn do_calculation(num_stack: &mut Vec<i32>, operator: char) {
    let rhs = num_stack.pop().unwrap();
    let lhs = num_stack.pop().unwrap();
    let result = match operator {
        '+' => lhs + rhs,
        '-' => lhs - rhs,
        '*' => lhs * rhs,
        '/' => lhs / rhs,
        _ => unreachable!(),
    };
    num_stack.push(result);
}

fn priority(operator: char) -> i32 {
    match operator {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => unreachable!(),
    }
}

#[test]
fn test() {
    let cases = vec![
        ("3+2*2", 7),
        (" 3/2 ", 1),
        (" 3+5 / 2 ", 5),
        ("1-1+1", 1),
        ("1*2-3/4+5*6-7*8+9/10", -24),
    ];
    for (input, expected) in cases {
        assert_eq!(calculate(input.to_string()), expected);
    }
}
