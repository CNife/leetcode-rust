pub fn balanced_string_split(s: impl AsRef<str>) -> i32 {
    let mut flag = 0;
    let mut cnt = 0;
    for ch in s.as_ref().bytes() {
        flag += match ch {
            b'L' => 1,
            b'R' => -1,
            _ => unreachable!(),
        };
        if flag == 0 {
            cnt += 1;
        }
    }
    cnt
}

#[test]
fn test_balanced_string_split() {
    let cases = vec![("RLRRLLRLRL", 4), ("RLLLLRRRLR", 3), ("LLLLRRRR", 1)];
    for (input, expected) in cases {
        let output = balanced_string_split(input);
        assert_eq!(output, expected);
    }
}
