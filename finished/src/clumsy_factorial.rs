pub fn clumsy(mut n: i32) -> i32 {
    let mut stack = Vec::new();
    stack.push(n);
    n -= 1;

    for i in 0..n {
        let num = match i % 4 {
            0 => stack.pop().unwrap() * n,
            1 => stack.pop().unwrap() / n,
            2 => n,
            3 => -n,
            _ => unreachable!(),
        };
        stack.push(num);
        n -= 1;
    }

    stack.into_iter().sum()
}

#[test]
fn test() {
    let tests = vec![(4, 7), (10, 12)];
    for (n, want) in tests {
        assert_eq!(clumsy(n), want);
    }
}
