//pub fn num_squares(n: i32) -> i32 {
//    let n = n as usize;
//    let mut dp = vec![0; n + 2];
//    for i in 1usize.. {
//        let index = i * i;
//        if index <= n {
//            dp[index] = 1;
//        } else {
//            break;
//        }
//    }
//    solve(n, &mut dp)
//}
//
//fn solve(n: usize, dp: &mut [i32]) -> i32 {
//    if dp[n] == 0 {
//        let max = (n as f64).sqrt().floor() as usize;
//        dp[n] = (1..=max).rev().map(|i| solve(n - i * i, dp)).min().unwrap() + 1;
//    }
//    println!("n = {}, dp[n] = {}", n, dp[n]);
//    dp[n]
//}

pub fn num_squares(n: i32) -> i32 {
    if is_square(n) {
        1
    } else if can_be_split_to_4_squares(n) {
        4
    } else if can_be_split_to_2_squares(n) {
        2
    } else {
        3
    }
}

fn is_square(n: i32) -> bool {
    let s = (n as f64).sqrt().floor() as i32;
    s * s == n
}

fn can_be_split_to_2_squares(n: i32) -> bool {
    (1..=(n as f64).sqrt().floor() as i32).any(|i| is_square(n - i * i))
}

fn can_be_split_to_4_squares(mut n: i32) -> bool {
    while n % 4 == 0 {
        n /= 4;
    }
    n % 8 == 7
}

#[test]
fn test() {
    let cases = vec![(12, 3), (13, 2)];
    for (n, expected) in cases {
        assert_eq!(num_squares(n), expected);
    }
}
