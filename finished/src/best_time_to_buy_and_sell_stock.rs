use std::cmp::{max, min};

pub fn max_profit(prices: &[i32]) -> i32 {
    if prices.len() < 2 {
        return 0;
    }

    let mut min_price = prices[0];
    let mut max_profit = 0;
    for &price in &prices[1..] {
        max_profit = max(max_profit, price - min_price);
        min_price = min(min_price, price);
    }

    max_profit
}

#[test]
fn test() {
    let cases = vec![(vec![7, 1, 5, 3, 6, 4], 5), (vec![7, 6, 4, 3, 1], 0)];
    for (prices, expected) in cases {
        assert_eq!(max_profit(&prices), expected);
    }
}
