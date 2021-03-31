pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    match candies.iter().max() {
        Some(&max) => candies
            .into_iter()
            .map(|candy| candy + extra_candies >= max)
            .collect(),
        None => vec![],
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![2, 3, 5, 1, 3], 3, vec![true, true, true, false, true]),
        (
            vec![4, 2, 1, 1, 2],
            1,
            vec![true, false, false, false, false],
        ),
        (vec![12, 1, 12], 10, vec![true, false, true]),
    ];
    for (candies, extra_candies, expect) in cases {
        assert_eq!(kids_with_candies(candies, extra_candies), expect);
    }
}
