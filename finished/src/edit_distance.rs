use std::cmp::min;
use std::iter::FromIterator;

pub fn min_distance(word1: String, word2: String) -> i32 {
    let lhs = word1.into_bytes();
    let rhs = word2.into_bytes();
    let m = lhs.len();
    let n = rhs.len();

    let mut dp = Vec::with_capacity(m + 1);
    dp.push(Vec::from_iter(0..=n as i32));
    for i in 1..=m as i32 {
        let mut row = vec![0; n + 1];
        row[0] = i;
        dp.push(row);
    }

    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if lhs[i - 1] == rhs[j - 1] {
                dp[i - 1][j - 1]
            } else {
                1 + min(dp[i - 1][j - 1], min(dp[i][j - 1], dp[i - 1][j]))
            };
        }
    }
    dp[m][n]
}

#[test]
fn test() {
    let cases = vec![("horse", "ros", 3), ("intention", "execution", 5)];
    for (lhs, rhs, expected) in cases {
        let output = min_distance(lhs.to_owned(), rhs.to_owned());
        assert_eq!(output, expected);
    }
}
