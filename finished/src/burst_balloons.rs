use std::cmp::max;

pub fn max_coins(mut nums: Vec<i32>) -> i32 {
    nums.reserve(2);
    nums.insert(0, 1);
    nums.push(1);

    let n = nums.len();
    let mut dp = vec![vec![0; n]; n];
    for k in 2..n {
        for i in 0..n - k {
            let j = i + k;
            for t in i + 1..j {
                dp[i][j] = max(dp[i][j], nums[i] * nums[t] * nums[j] + dp[i][t] + dp[t][j]);
            }
        }
    }
    dp[0][n - 1]
}

#[test]
fn test() {
    let cases = vec![(vec![3, 1, 5, 8], 167)];
    for (nums, expected) in cases {
        assert_eq!(max_coins(nums), expected);
    }
}
