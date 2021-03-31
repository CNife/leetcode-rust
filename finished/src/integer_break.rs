pub fn integer_break(mut n: i32) -> i32 {
    if n < 2 {
        return 0;
    } else if n < 4 {
        return n - 1;
    }

    let mut product = 1;
    while n > 4 {
        n -= 3;
        product *= 3;
    }
    product * n
}

#[test]
fn test() {
    let tests = vec![(2, 1), (10, 36)];
    for (n, want) in tests {
        assert_eq!(integer_break(n), want);
    }
}
