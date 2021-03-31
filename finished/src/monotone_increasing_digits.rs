pub fn monotone_increasing_digits(mut n: i32) -> i32 {
    let mut result = 0;
    let mut seed = 1;
    while n > 0 {
        let num = n % 10;
        n /= 10;
        let high = n % 10;
        if high > num {
            result = seed * 10 - 1;
            n -= 1;
        } else {
            result += num * seed;
        }
        seed *= 10;
    }
    result
}

#[test]
fn test() {
    let tests = vec![(10, 9), (1234, 1234), (332, 299)];
    for (n, want) in tests {
        assert_eq!(monotone_increasing_digits(n), want);
    }
}
