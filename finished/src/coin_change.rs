// 简单的动态规划解法 16ms 2MB
// use std::cmp::min;
// use std::i32;
//
// pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
//     if amount == 0 {
//         return 0;
//     }
//
//     let amount = amount as usize;
//     let coins: Vec<usize> = coins.into_iter().map(|n| n as usize).collect();
//
//     let mut dp = vec![0; amount + 1];
//     for &coin in &coins {
//         if coin <= amount {
//             dp[coin] = 1;
//         }
//     }
//
//     for i in 1..=amount {
//         if dp[i] == 0 {
//             for &coin in &coins {
//                 if coin < i && dp[i - coin] > 0 {
//                     let change = dp[i - coin] + 1;
//                     dp[i] = if dp[i] == 0 {
//                         change
//                     } else {
//                         min(dp[i], change)
//                     };
//                 }
//             }
//         }
//     }
//     if dp[amount] == 0 {
//         -1
//     } else {
//         dp[amount]
//     }
// }

use std::cmp::{min, Reverse};

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    if amount == 0 {
        return 0;
    }

    let mut coins: Vec<usize> = coins
        .into_iter()
        .filter(|n| *n <= amount)
        .map(|n| n as usize)
        .collect();
    coins.sort_unstable_by_key(|coin| Reverse(*coin));
    let amount = amount as usize;
    let mut table = vec![0; amount + 1];
    for &coin in &coins {
        table[coin] = 1;
    }

    if dfs(&coins, amount, &mut table) <= 0 {
        -1
    } else {
        table[amount]
    }
}

fn dfs(coins: &[usize], amount: usize, table: &mut Vec<i32>) -> i32 {
    if table[amount] == 0 {
        let mut min_change = std::i32::MAX;
        for &coin in coins {
            if amount > coin {
                let next_change = dfs(coins, amount - coin, table);
                if next_change > 0 {
                    min_change = min(min_change, next_change + 1);
                }
            }
        }
        table[amount] = if min_change == std::i32::MAX {
            -1
        } else {
            min_change
        };
    }
    table[amount]
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 5], 11, 3),
        (vec![2], 3, -1),
        (vec![1], 0, 0),
        (vec![1], 1, 1),
    ];
    for (coins, amount, expected) in cases {
        assert_eq!(coin_change(coins, amount), expected);
    }
}
