pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    debug_assert!(n > 0 && k > 0 && n >= k);

    let n = n as usize;
    let k = k as usize;
    let mut results = Vec::new();
    let mut stack = Vec::with_capacity(k);

    backtrack(1, n, k, &mut stack, &mut results);
    results
}

fn backtrack(
    start: usize,
    end: usize,
    k: usize,
    stack: &mut Vec<i32>,
    results: &mut Vec<Vec<i32>>,
) {
    if stack.len() < k {
        for num in start..=end {
            stack.push(num as i32);
            backtrack(num + 1, end, k, stack, results);
            stack.pop();
        }
    } else {
        results.push(stack.clone());
    }
}

#[test]
fn test() {
    use utils::assert_same_set;

    let cases = vec![(
        4,
        2,
        vec![
            vec![2, 4],
            vec![3, 4],
            vec![2, 3],
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
        ],
    )];
    for (n, k, expected) in cases {
        assert_same_set(combine(n, k), expected);
    }
}
