pub fn number_of_lines<S: AsRef<[u8]>>(widths: Vec<i32>, s: S) -> Vec<i32> {
    let mut lines = 0;
    let mut line_cnt = 0;
    for ch in s.as_ref() {
        let cnt = widths[(*ch - b'a') as usize];
        if cnt + line_cnt > 100 {
            lines += 1;
            line_cnt = cnt;
        } else {
            line_cnt += cnt;
        }
    }
    vec![if line_cnt == 100 { lines } else { lines + 1 }, line_cnt]
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![
                10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                10, 10, 10, 10, 10,
            ],
            "abcdefghijklmnopqrstuvwxyz",
            vec![3, 60],
        ),
        (
            vec![
                4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                10, 10, 10, 10, 10,
            ],
            "bbbcccdddaaa",
            vec![2, 4],
        ),
    ];
    for (widths, s, want) in cases {
        assert_eq!(number_of_lines(widths, s), want);
    }
}
