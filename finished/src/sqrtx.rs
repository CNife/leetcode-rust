pub fn my_sqrt(x: i32) -> i32 {
    (x as f64).sqrt().floor() as i32
}

#[test]
fn test() {
    let cases = vec![(0, 0), (1, 1), (4, 2), (8, 2), (2147395599, 46339)];
    for (x, expect) in cases {
        assert_eq!(my_sqrt(x), expect);
    }
}
