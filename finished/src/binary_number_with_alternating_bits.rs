pub fn has_alternating_bits(n: i32) -> bool {
    let mut num = 0x55555555 as i32;
    while num != 0 {
        if n == num {
            return true;
        } else {
            num >>= 1;
        }
    }
    false
}

#[test]
fn test() {
    let cases = vec![(5, true), (7, false), (11, false)];
    for (n, expected) in cases {
        assert_eq!(has_alternating_bits(n), expected);
    }
}
