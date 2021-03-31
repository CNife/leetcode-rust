use std::cmp::{max, min, Ordering::*};
use std::collections::HashMap;

pub fn super_egg_drop(k: i32, n: i32) -> i32 {
    let mut memo = HashMap::new();
    dp(&mut memo, k, n)
}

fn dp(memo: &mut HashMap<(i32, i32), i32>, k: i32, n: i32) -> i32 {
    debug_assert!(k > 0);
    debug_assert!(n >= 0);

    match memo.get(&(k, n)) {
        Some(result) => *result,
        None => {
            let result = match (k, n) {
                (_, 0) => 0,
                (1, n) => n,
                (k, n) => {
                    let mut lo = 1;
                    let mut hi = n;
                    while lo + 1 < hi {
                        let x = (lo + hi) / 2;
                        let t1 = dp(memo, k - 1, x - 1);
                        let t2 = dp(memo, k, n - x);
                        match t1.cmp(&t2) {
                            Equal => {
                                lo = x;
                                hi = x;
                            }
                            Less => lo = x,
                            Greater => hi = x,
                        }
                    }
                    1 + min(
                        max(dp(memo, k - 1, lo - 1), dp(memo, k, n - lo)),
                        max(dp(memo, k - 1, hi - 1), dp(memo, k, n - hi)),
                    )
                }
            };
            memo.insert((k, n), result);
            result
        }
    }
}

#[test]
fn test() {
    let cases = vec![(1, 2, 2), (2, 6, 3), (3, 14, 4), (3, 25, 5)];
    for (k, n, expect) in cases {
        assert_eq!(super_egg_drop(k, n), expect);
    }
}
