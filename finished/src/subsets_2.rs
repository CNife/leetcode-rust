pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let mut stack = Vec::with_capacity(nums.len());
    let mut results = Vec::new();
    backtrack(&nums, &mut stack, &mut results);
    results
}

fn backtrack(nums: &[i32], stack: &mut Vec<i32>, results: &mut Vec<Vec<i32>>) {
    results.push(stack.clone());
    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        stack.push(nums[i]);
        backtrack(&nums[i + 1..], stack, results);
        stack.pop();
    }
}

#[test]
fn test() {
    use utils::assert_same_set;

    let cases = vec![(
        vec![1, 2, 2],
        vec![
            vec![2],
            vec![1],
            vec![1, 2, 2],
            vec![2, 2],
            vec![1, 2],
            vec![],
        ],
    )];
    for (nums, expected) in cases {
        let output = subsets_with_dup(nums);
        assert_same_set(output, expected);
    }
}
