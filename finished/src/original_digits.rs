impl Solution {
    pub fn original_digits(s: String) -> String {
        let mut map = CharMap::from_str(&s);
        let mut result = vec![];

        loop {
            let digit = map.next_digit();
            if digit > 9 {
                break;
            } else {
                map.minus(digit);
                result.push(DIGITS[digit]);
            }
        }

        result.sort_unstable();
        result.into_iter().collect::<String>()
    }
}

struct CharMap([u32; 26]);

impl CharMap {
    fn from_str(s: &str) -> CharMap {
        let mut map = [0u32; 26];
        for c in s.as_bytes() {
            let idx = *c as u8 - b'a';
            map[idx as usize] += 1;
        }
        CharMap(map)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.0 == [0; 26]
    }

    fn next_digit(&self) -> usize {
        if self.is_empty() {
            return 10;
        }
        for &(idx, digit) in &DIGIT_EXISTS_MAP {
            if self.0[idx] > 0 {
                return digit;
            }
        }
        unreachable!();
    }

    fn minus(&mut self, digit: usize) {
        for i in 0usize..26 {
            self.0[i] -= DIGITS_MAP[digit as usize][i];
        }
    }
}

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

const DIGIT_EXISTS_MAP: [(usize, usize); 10] = [
    (25, 0),
    (22, 2),
    (23, 6),
    (20, 4),
    (17, 3),
    (18, 7),
    (14, 1),
    (21, 5),
    (19, 8),
    (8, 9),
];

const DIGITS_MAP: [[u32; 26]; 10] = [
    [
        0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 2, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0,
    ],
    [
        0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
];

pub struct Solution;

#[test]
fn test_original_digits() {
    let cases = vec![
        ("owoztneoer", "012"),
        ("fviefuro", "45"),
        ("zerozero", "00"),
    ];
    for (s, expected) in cases {
        assert_eq!(Solution::original_digits(s.to_string()), expected);
    }
}
