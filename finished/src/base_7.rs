pub fn convert_to_base7(num: i32) -> String {
    if num == 0 {
        String::from("0")
    } else {
        let mut buffer = positive_to_base7(if num > 0 { num } else { -num });
        if num < 0 {
            buffer.push(b'-');
        }
        buffer.reverse();
        String::from_utf8(buffer).unwrap()
    }
}

fn positive_to_base7(num: i32) -> Vec<u8> {
    let mut num = num;
    let mut result = Vec::new();
    while num != 0 {
        let digit = b'0' + (num % 7) as u8;
        result.push(digit);
        num /= 7;
    }
    result
}

#[test]
fn test() {
    let cases = vec![(100, "202"), (-7, "-10"), (0, "0")];
    for (num, expected) in cases {
        assert_eq!(convert_to_base7(num), expected);
    }
}
