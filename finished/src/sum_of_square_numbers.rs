pub fn judge_square_sum(num: i32) -> bool {
    debug_assert!(num > 0);

    let bound = (num as f64 / 2.0).sqrt().floor() as i32;
    for a in 0..=bound {
        let b = ((num - a * a) as f64).sqrt();
        if (b.floor() - b.ceil()).abs() < f64::EPSILON {
            return true;
        }
    }
    false
}

#[test]
fn test() {
    let cases = vec![(5, true), (3, false), (4, true)];
    for (num, expected) in cases {
        assert_eq!(judge_square_sum(num), expected);
    }
}
