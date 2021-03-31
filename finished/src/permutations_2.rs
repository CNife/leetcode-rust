pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();

    let mut used = vec![false; nums.len()];
    let mut path = Vec::with_capacity(nums.len());
    let mut result = vec![];
    dfs(&nums, 0, used.as_mut_slice(), &mut path, &mut result);
    result
}

fn dfs(nums: &[i32], n: usize, used: &mut [bool], path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if n == nums.len() {
        result.push(path.clone());
    } else {
        let mut prev = 0;
        let mut is_start = true;
        for i in 0..nums.len() {
            if (!is_start && prev == nums[i]) || used[i] {
                continue;
            }
            path.push(nums[i]);
            used[i] = true;
            dfs(nums, n + 1, used, path, result);
            used[i] = false;
            path.pop().unwrap();
            is_start = false;
            prev = nums[i];
        }
    }
}

#[test]
fn test() {
    let cases = vec![(
        vec![1, 1, 2],
        vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]],
    )];
    for (nums, expected) in cases {
        assert_eq!(permute_unique(nums), expected);
    }
}
