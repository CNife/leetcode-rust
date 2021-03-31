pub struct Solution;

impl Solution {
    pub fn multiply(a: String, b: String) -> String {
        let (a, b) = Solution::init(a, b);
        if Solution::is_zero(&a) || Solution::is_zero(&b) {
            return "0".to_string();
        }

        let mut res = Vec::with_capacity(a.len() + b.len());
        let mut buf = Vec::with_capacity(a.len() + 1);
        for (i, digit) in b.into_iter().enumerate() {
            Solution::multiply_digit(&a, digit, &mut buf);
            Solution::add_line(&mut res, &buf, i);
        }
        Solution::finish_to_string(res)
    }

    fn init(a: String, b: String) -> (Vec<u8>, Vec<u8>) {
        let to_reversed_digits = |s: String| {
            s.into_bytes()
                .into_iter()
                .map(|c| c - b'0')
                .rev()
                .collect::<Vec<u8>>()
        };

        let a_vec = to_reversed_digits(a);
        let b_vec = to_reversed_digits(b);
        if a_vec.len() < b_vec.len() {
            (a_vec, b_vec)
        } else {
            (b_vec, a_vec)
        }
    }

    fn is_zero(s: &[u8]) -> bool {
        s.len() == 1 && s[0] == 0
    }

    fn multiply_digit(a: &[u8], rhs: u8, buf: &mut Vec<u8>) {
        buf.clear();
        let mut carry = 0u8;
        for &lhs in a {
            let result = lhs * rhs + carry;
            carry = result / 10;
            buf.push(result % 10);
        }
        if carry > 0 {
            buf.push(carry);
        }
    }

    fn add_line(res: &mut Vec<u8>, buf: &[u8], offset: usize) {
        let mut carry = 0u8;
        let mut add_with_carry = |lhs: u8, rhs: u8| -> u8 {
            let sum = lhs + rhs + carry;
            if sum > 9 {
                carry = 1;
                sum - 10
            } else {
                carry = 0;
                sum
            }
        };

        for (lhs, rhs) in res.iter_mut().skip(offset).zip(buf.iter()) {
            *lhs = add_with_carry(*lhs, *rhs);
        }
        if buf.len() + offset <= res.len() {
            for lhs in &mut res[offset + buf.len()..] {
                *lhs = add_with_carry(*lhs, 0);
            }
        } else {
            for rhs in &buf[res.len() - offset..] {
                res.push(add_with_carry(0, *rhs));
            }
        }
        if carry > 0 {
            res.push(1);
        }
    }

    fn finish_to_string(res: Vec<u8>) -> String {
        let tmp = res.into_iter().map(|d| d + b'0').rev().collect::<Vec<_>>();
        unsafe { String::from_utf8_unchecked(tmp) }
    }
}

#[test]
fn test_multiply() {
    let cases = vec![("2", "3", "6"), ("123", "456", "56088")];
    for (a, b, expected) in cases {
        assert_eq!(Solution::multiply(a.to_string(), b.to_string()), expected);
    }
}
