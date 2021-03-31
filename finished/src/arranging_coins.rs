pub fn arrange_coins(n: i32) -> i32 {
    //    let mut line = 1;
    //    let mut sum = 1;
    //    while sum <= n {
    //        line += 1;
    //        match sum.checked_add(line) {
    //            None => break,
    //            Some(s) => sum = s,
    //        }
    //    }
    //    line - 1
    (2.0f64.sqrt() * (n as f64 + 0.125).sqrt() - 0.5) as i32
}

#[test]
fn test() {
    let cases = vec![(5, 2), (8, 3), (1, 1), (2, 1), (3, 2), (0, 0)];
    for (n, expected) in cases {
        assert_eq!(arrange_coins(n), expected);
    }
}
