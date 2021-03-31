use std::collections::HashMap;

pub fn fraction_to_decimal(up: i32, down: i32) -> String {
    if down == 0 {
        return "".into();
    }
    if up == 0 {
        return "0".into();
    }

    let up = up as i64;
    let down = down as i64;
    let mut buf = String::new();

    let int = up / down;
    if int == 0 && (up < 0) ^ (down < 0) {
        buf.push('-');
    }
    buf.push_str(int.to_string().as_str());

    let up = up.abs();
    let down = down.abs();
    let mut rem = up % down;
    if rem == 0 {
        return buf;
    }
    buf.push('.');

    let mut map: HashMap<i64, usize> = HashMap::new();
    while rem != 0 {
        match map.get(&rem) {
            Some(&i) => {
                buf.insert(i, '(');
                buf.push(')');
                break;
            }
            None => {
                map.insert(rem, buf.len());
                rem *= 10;
                buf.push_str((rem / down).to_string().as_str());
                rem %= down;
            }
        }
    }
    buf
}

#[test]
fn test() {
    let cases = vec![
        (1, 2, "0.5"),
        (2, 1, "2"),
        (2, 3, "0.(6)"),
        (-1, 7, "-0.(142857)"),
        (std::i32::MIN, -1, "2147483648"),
    ];
    for (up, down, expect) in cases {
        assert_eq!(fraction_to_decimal(up, down), expect);
    }
}
