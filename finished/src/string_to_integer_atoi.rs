use std::i32;

pub fn my_atoi(s: String) -> i32 {
    let mut iter = s.chars();
    let mut is_positive = true;
    let mut result = 0i64;

    while let Some(ch) = iter.next() {
        match ch {
            ' ' => continue,
            '+' => break,
            '-' => {
                is_positive = false;
                break;
            }
            ch if ch.is_ascii_digit() => {
                result = char_to_digit(ch);
                break;
            }
            _ => return 0,
        }
    }

    for ch in iter {
        if ch.is_ascii_digit() {
            result = result * 10 + char_to_digit(ch);
            if result > i32::MAX as i64 && is_positive {
                return i32::MAX;
            } else if result > (i32::MAX as i64 + 1) && !is_positive {
                return i32::MIN;
            }
        } else {
            break;
        }
    }

    (if is_positive { result } else { -result }) as i32
}

#[inline]
fn char_to_digit(ch: char) -> i64 {
    ((ch as u8) - b'0') as i64
}

#[test]
fn test() {
    let cases = vec![
        ("42", 42),
        ("   -42", -42),
        ("4193 with words", 4193),
        ("words and 987", 0),
        ("-91283472332", -2147483648),
        ("-+1", 0),
    ];
    for (str, expected) in cases {
        assert_eq!(my_atoi(str.to_string()), expected);
    }
}
