pub fn add_strings(lhs: String, rhs: String) -> String {
    let (mut lhs, mut rhs) = {
        let u8_iter = |s: String| s.into_bytes().into_iter().rev();
        (u8_iter(lhs), u8_iter(rhs))
    };
    let mut next = || match (lhs.next(), rhs.next()) {
        (None, None) => None,
        (l, r) => {
            let unwrap = |n: Option<u8>| n.unwrap_or(b'0') - b'0';
            Some((unwrap(l), unwrap(r)))
        }
    };

    let mut sum = Vec::new();
    let mut carry = false;
    while let Some((l, r)) = next() {
        let mut s = l + r;
        if carry {
            s += 1;
        }
        if s > 9 {
            s -= 10;
            carry = true;
        } else {
            carry = false;
        }
        sum.push(s + b'0');
    }
    if carry {
        sum.push(b'1');
    }
    sum.reverse();
    String::from_utf8(sum).unwrap()
}

#[test]
fn test() {
    let cases = vec![(1234, 5678), (999999, 999999), (9133, 0)];
    for (lhs, rhs) in cases {
        let sum = lhs + rhs;
        assert_eq!(
            add_strings(lhs.to_string(), rhs.to_string()),
            sum.to_string()
        );
    }
}
