pub fn max_depth_after_split(input: String) -> Vec<i32> {
    let mut result = Vec::with_capacity(input.len());
    for (i, byte) in input.into_bytes().into_iter().enumerate() {
        let result_bit = if byte == b'(' { i & 1 } else { (i + 1) & 1 };
        result.push(result_bit as i32);
    }
    result
}

#[test]
fn test() {
    use std::cmp::max;

    fn max_depth(s: &str, split: Vec<i32>) -> i32 {
        let (mut lhs_depth, mut lhs_counter) = (0, 0);
        let (mut rhs_depth, mut rhs_counter) = (0, 0);
        for item in s.bytes().zip(split.into_iter()) {
            match item {
                (b'(', 0) => {
                    lhs_counter += 1;
                    lhs_depth = max(lhs_depth, lhs_counter);
                }
                (b')', 0) => {
                    lhs_counter -= 1;
                    lhs_depth = max(lhs_depth, -lhs_counter);
                }
                (b'(', 1) => {
                    rhs_counter += 1;
                    rhs_depth = max(rhs_depth, rhs_counter);
                }
                (b')', 1) => {
                    rhs_counter -= 1;
                    rhs_depth = max(rhs_depth, -rhs_counter);
                }
                _ => unreachable!(),
            }
        }
        max(lhs_depth, rhs_depth)
    }

    let tests = vec![
        ("(()())", vec![0, 1, 1, 1, 1, 0]),
        ("()(())()", vec![0, 0, 0, 1, 1, 0, 1, 1]),
    ];
    for (input, want_example) in tests {
        assert_eq!(
            max_depth(input, max_depth_after_split(input.into())),
            max_depth(input, want_example)
        );
    }
}
