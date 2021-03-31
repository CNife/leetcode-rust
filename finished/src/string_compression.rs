pub fn compress(chars: &mut Vec<char>) -> i32 {
    let mut left = 0;
    let mut right = 0;
    while right < chars.len() {
        let last_right = right;
        let start = chars[right];
        while right < chars.len() && chars[right] == start {
            right += 1;
        }

        let mut set_char = |ch: char| {
            chars[left] = ch;
            left += 1;
        };
        let count = right - last_right;
        set_char(start);
        if count > 1 {
            for ch in count.to_string().chars() {
                set_char(ch);
            }
        }
    }
    left as i32
}

#[test]
fn test() {
    let cases = vec![
        (
            vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'],
            vec!['a', '2', 'b', '2', 'c', '3'],
        ),
        (vec!['a'], vec!['a']),
        (
            vec![
                'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
            ],
            vec!['a', 'b', '1', '2'],
        ),
        (
            vec!['a', 'a', 'a', 'b', 'b', 'a', 'a'],
            vec!['a', '3', 'b', '2', 'a', '2'],
        ),
    ];
    for (mut src, expected_bytes) in cases {
        assert_eq!(compress(&mut src), expected_bytes.len() as i32);
        assert_eq!(src[..expected_bytes.len()], expected_bytes[..]);
    }
}
