pub fn last_remaining(n: i32, k: i32) -> i32 {
    let mut result = 0;
    for i in 2..=n {
        result = (result + k) % i;
    }
    result
}

#[test]
fn test() {
    let cases = vec![(5, 3, 3), (10, 17, 2)];
    for (n, k, expect) in cases {
        assert_eq!(last_remaining(n, k), expect);
    }
}
