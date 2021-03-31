const INVALID: i8 = -1;
const START: i8 = 0;
const REMAIN_1: i8 = 1;
const REMAIN_2: i8 = 2;
const REMAIN_3: i8 = 3;

pub fn valid_utf8(data: Vec<i32>) -> bool {
    let mut state = START;
    for byte in data {
        state = read_byte(state, byte as u8);
        if state == INVALID {
            return false;
        }
    }
    state == START
}

fn read_byte(state: i8, byte: u8) -> i8 {
    match state {
        START => {
            if byte >= 0b1111_1000 {
                INVALID
            } else if byte >= 0b1111_0000 {
                REMAIN_3
            } else if byte >= 0b1110_0000 {
                REMAIN_2
            } else if byte >= 0b1100_0000 {
                REMAIN_1
            } else if byte >= 0b1000_0000 {
                INVALID
            } else {
                START
            }
        }
        s if s > 0 => {
            if byte >= 0b1000_0000 && byte < 0b1100_0000 {
                s - 1
            } else {
                INVALID
            }
        }
        _ => unreachable!(),
    }
}

#[test]
fn test() {
    let cases = vec![(vec![197, 130, 1], true), (vec![235, 140, 4], false)];
    for (data, expected) in cases {
        assert_eq!(valid_utf8(data), expected);
    }
}
