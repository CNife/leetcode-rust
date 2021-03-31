pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }
    let s = s.into_bytes();
    let n = num_rows as usize;
    let step = (n - 1) * 2;
    let mut result = Vec::with_capacity(s.len());
    for r in 0..n {
        let mut idx = r;
        if r == 0 || r == n - 1 {
            while idx < s.len() {
                result.push(s[idx]);
                idx += step;
            }
        } else {
            let mut flag = false;
            let sub_step = r * 2;
            while idx < s.len() {
                result.push(s[idx]);
                idx += if flag { sub_step } else { step - sub_step };
                flag = !flag;
            }
        }
    }
    unsafe { String::from_utf8_unchecked(result) }
}

#[test]
fn test() {
    let cases = vec![
        //        ("LEETCODEISHIRING", 3, "LCIRETOESIIGEDHN"),
        //        ("LEETCODEISHIRING", 4, "LDREOEIIECIHNTSG"),
        ("A", 1, "A"),
    ];
    for (s, n, expected) in cases {
        assert_eq!(convert(s.to_string(), n), expected);
    }
}
