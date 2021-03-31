pub fn min_moves(nums: Vec<i32>) -> i32 {
    match nums.iter().min() {
        Some(&min) => nums.into_iter().map(|n| n - min).sum(),
        None => 0,
    }
}

#[test]
fn test() {
    let cases = vec![(vec![1, 2, 3], 3)];
    for (input, expected) in cases {
        assert_eq!(min_moves(input), expected);
    }
}
