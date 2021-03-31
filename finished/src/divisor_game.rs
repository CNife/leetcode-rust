pub fn divisor_game(n: i32) -> bool {
    let n = n as usize;
    let mut win = vec![false; if n == 1 { 3 } else { n + 1 }];
    win[2] = true;

    for i in 3..=n {
        for j in 1..=i / 2 {
            if i % j == 0 && !win[i - j] {
                win[i] = true;
            }
        }
    }
    win[n]
}

#[test]
fn test_divisor_game() {
    let tests = vec![(2, true), (3, false)];
    for (n, want) in tests {
        assert_eq!(divisor_game(n), want);
    }
}
