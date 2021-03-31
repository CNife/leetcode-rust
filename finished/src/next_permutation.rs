pub fn next_permutation(nums: &mut Vec<i32>) {
    match (0..nums.len() - 1).rfind(|&i| nums[i] < nums[i + 1]) {
        None => {
            nums.reverse();
        }
        Some(m) => {
            let n = (m + 1..nums.len()).rfind(|&i| nums[i] > nums[m]).unwrap();
            nums.swap(m, n);
            nums[m + 1..].reverse();
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 3], vec![1, 3, 2]),
        (vec![3, 2, 1], vec![1, 2, 3]),
        (vec![1, 1, 5], vec![1, 5, 1]),
    ];
    for (mut input, expected) in cases {
        next_permutation(&mut input);
        assert_eq!(input, expected);
    }
}
