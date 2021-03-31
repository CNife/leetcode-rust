pub fn decode_ways(input: String) -> i32 {
    let input = input.as_bytes();
    if input.is_empty() || input[0] == b'0' {
        return 0;
    }

    let mut prev = 1;
    let mut curr = 1;
    for i in 1..input.len() {
        let tmp = curr;
        if input[i] == b'0' {
            if input[i - 1] == b'1' || input[i - 1] == b'2' {
                curr = prev;
            } else {
                return 0;
            }
        } else if input[i - 1] == b'1'
            || input[i - 1] == b'2' && input[i] >= b'0' && input[i] <= b'6'
        {
            curr += prev;
        }
        prev = tmp;
    }
    curr
}

#[test]
fn test() {
    let cases = vec![("12", 2), ("226", 3)];
    for (input, expected) in cases {
        assert_eq!(decode_ways(input.to_string()), expected);
    }
}
