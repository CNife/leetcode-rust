use std::cmp::min;

pub fn nth_ugly_number(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let n = n as usize;
    let mut dp = vec![0; n];
    dp[0] = 1;

    let (mut p2, mut p3, mut p5) = (0, 0, 0);
    for i in 1..n {
        let (n2, n3, n5) = (2 * dp[p2], 3 * dp[p3], 5 * dp[p5]);
        dp[i] = min(min(n2, n3), n5);
        if dp[i] == n2 {
            p2 += 1;
        }
        if dp[i] == n3 {
            p3 += 1;
        }
        if dp[i] == n5 {
            p5 += 1;
        }
    }

    dp[n - 1]
}

#[test]
fn test() {
    let tests = vec![(10, 12), (1, 1)];
    for (n, want) in tests {
        assert_eq!(nth_ugly_number(n), want);
    }
}
