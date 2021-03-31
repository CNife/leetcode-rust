pub fn gcd_of_strings(lhs: String, rhs: String) -> String {
    if lhs.clone() + &rhs == rhs.clone() + &lhs {
        lhs[..gcd(lhs.len(), rhs.len())].to_string()
    } else {
        String::new()
    }
}

fn gcd(lhs: usize, rhs: usize) -> usize {
    if rhs == 0 {
        lhs
    } else {
        gcd(rhs, lhs % rhs)
    }
}

#[test]
fn test() {
    let cases = vec![
        ("ABCABC", "ABC", "ABC"),
        ("ABABAB", "ABAB", "AB"),
        ("LEET", "CODE", ""),
    ];
    for (str1, str2, expected) in cases {
        assert_eq!(gcd_of_strings(str1.to_string(), str2.to_string()), expected);
    }
}
