pub fn count_prime_set_bits(l: i32, r: i32) -> i32 {
    (l..=r)
        .map(|n| n.count_ones())
        .map(|cnt| PRIME[cnt as usize])
        .sum()
}

const PRIME: [i32; 33] = [
    0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1,
    0,
];

#[test]
fn test_count_prime_set_bits() {
    let cases = vec![(6, 10, 4), (10, 15, 5)];
    for (l, r, expected) in cases {
        let output = count_prime_set_bits(l, r);
        assert_eq!(output, expected);
    }
}
