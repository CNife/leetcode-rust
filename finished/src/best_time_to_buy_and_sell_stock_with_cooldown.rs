use std::cmp::max;

pub fn max_profit(prices: Vec<i32>) -> i32 {
    //    let n = prices.len();
    //    let mut dp = vec![[0; 3]; n + 1];
    //    dp[0][1] = std::i32::MIN;
    //
    //    for i in 0..n {
    //        dp[i + 1][0] = max(dp[i][0], dp[i][2]);
    //        dp[i + 1][1] = max(dp[i][1], dp[i][0] - prices[i]);
    //        dp[i + 1][2] = dp[i][1] + prices[i];
    //    }
    //    max(dp[n][0], dp[n][2])

    let mut rest = 0;
    let mut hold = std::i32::MIN;
    let mut sold = 0;
    for price in prices {
        let old_sold = sold;
        sold = hold + price;
        hold = max(hold, rest - price);
        rest = max(rest, old_sold);
    }
    max(rest, sold)
}

#[test]
fn test() {
    let cases = vec![(vec![1, 2, 3, 0, 2], 3)];
    for (prices, expected) in cases {
        assert_eq!(max_profit(prices), expected);
    }
}
