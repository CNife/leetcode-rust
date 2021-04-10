use std::cmp::max;

pub fn longest_common_subsequence(text1: impl AsRef<str>, text2: impl AsRef<str>) -> i32 {
    let b1 = text1.as_ref().as_bytes();
    let b2 = text2.as_ref().as_bytes();
    let n1 = b1.len();
    let n2 = b2.len();

    let mut dp = vec![vec![0; n2 + 1]; n1 + 1];
    for i in 1..=n1 {
        for j in 1..=n2 {
            dp[i][j] = if b1[i - 1] == b2[j - 1] {
                dp[i - 1][j - 1] + 1
            } else {
                max(dp[i][j - 1], dp[i - 1][j])
            };
        }
    }
    dp[n1][n2]
}

#[test]
fn test() {
    let tests = vec![("abcde", "ace", 3), ("abc", "abc", 3), ("abc", "def", 0)];
    for (text1, text2, want) in tests {
        assert_eq!(longest_common_subsequence(text1, text2), want);
    }
}
