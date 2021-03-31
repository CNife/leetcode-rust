pub fn num_trees(n: i32) -> i32 {
    let n = n as i64;
    let mut c = 1;
    for i in 0..n {
        c = c * 2 * (2 * i + 1) / (i + 2);
    }
    c as i32
}

#[test]
fn test() {
    let cases = vec![(3, 5)];
    for (n, expected) in cases {
        assert_eq!(num_trees(n), expected);
    }
}
