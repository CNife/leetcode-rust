pub fn calculate(input: String) -> i32 {
    let mut input = input.as_str();
    let mut token_stack: Vec<Token> = Vec::new();
    let mut num_stack = Vec::new();

    while let (Some(token), remained_str) = next_token(input) {
        match token {
            Token::Number(num) => {
                num_stack.push(num);
                do_calculate(&mut token_stack, &mut num_stack);
            }
            Token::Operator(_) | Token::LeftParenthesis => {
                token_stack.push(token);
            }
            Token::RightParenthesis => {
                token_stack.pop().unwrap();
                do_calculate(&mut token_stack, &mut num_stack);
            }
        }
        input = remained_str;
    }

    num_stack.pop().unwrap()
}

#[derive(Copy, Clone, Debug)]
enum Token {
    Number(i32),
    Operator(char),
    LeftParenthesis,
    RightParenthesis,
}

fn do_calculate(token_stack: &mut Vec<Token>, num_stack: &mut Vec<i32>) {
    let mut pop_token_stack = false;
    if let Some(Token::Operator(op)) = token_stack.last() {
        pop_token_stack = true;
        let rhs = num_stack.pop().unwrap();
        let lhs = num_stack.pop().unwrap();
        let result = match op {
            '+' => lhs + rhs,
            '-' => lhs - rhs,
            _ => unreachable!(),
        };
        num_stack.push(result);
    }
    if pop_token_stack {
        token_stack.pop().unwrap();
    }
}

fn next_token(input: &str) -> (Option<Token>, &str) {
    let input = input.trim_start();
    match input.chars().next() {
        None => (None, input),
        Some(first) => {
            if first.is_digit(10) {
                let num_len = input
                    .char_indices()
                    .find(|(_, ch)| !ch.is_digit(10))
                    .map_or(input.len(), |(i, _)| i);
                let num = input[..num_len].parse::<i32>().unwrap();
                (Some(Token::Number(num)), &input[num_len..])
            } else {
                let token = match first {
                    '+' | '-' => Token::Operator(first),
                    '(' => Token::LeftParenthesis,
                    ')' => Token::RightParenthesis,
                    _ => unreachable!(),
                };
                (Some(token), &input[1..])
            }
        }
    }
}

#[test]
fn test() {
    let cases = vec![("1 + 1", 2), (" 2-1 + 2 ", 3), ("(1+(4+5+2)-3)+(6+8)", 23)];
    for (input, expected) in cases {
        assert_eq!(calculate(input.to_string()), expected);
    }
}
