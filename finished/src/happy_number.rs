use std::collections::HashSet;

pub fn is_happy(mut n: i32) -> bool {
    let mut set = HashSet::new();
    while !set.contains(&n) {
        if n == 1 {
            return true;
        }
        set.insert(n);
        n = transform(n);
    }
    false
}

fn transform(mut n: i32) -> i32 {
    let mut result = 0;
    while n > 0 {
        let digit = n % 10;
        result += digit * digit;
        n /= 10;
    }
    result
}

#[test]
fn test() {
    let cases = vec![(19, true)];
    for (n, expect) in cases {
        assert_eq!(is_happy(n), expect);
    }
}
