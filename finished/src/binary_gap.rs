use std::cmp::max;

pub fn binary_gap(n: i32) -> i32 {
    let mut last_idx = -1;
    let mut max_gap: Option<i32> = None;
    for i in 0..32 {
        if (n >> i) & 1 == 1 {
            if last_idx >= 0 {
                let gap = i - last_idx;
                max_gap = Some(match max_gap {
                    Some(mg) => max(mg, gap),
                    None => gap,
                });
            }
            last_idx = i;
        }
    }
    max_gap.unwrap_or(0)
}

#[test]
fn test() {
    let cases = vec![(22, 2), (5, 2), (6, 1), (8, 0)];
    for (n, expected) in cases {
        assert_eq!(binary_gap(n), expected);
    }
}
