use std::cmp;

pub fn array_nesting(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut visited = vec![false; n];
    let mut max = 0;
    for i in 0..n {
        if !visited[i] {
            let result = dfs_iter(&nums, i, &mut visited);
            max = cmp::max(max, result);
        }
    }
    max
}

// fn dfs(nums: &[i32], i: usize, visited: &mut Vec<bool>) -> i32 {
//     if visited[i] {
//         return 0;
//     }
//     visited[i] = true;
//     return dfs(nums, nums[i] as usize, visited) + 1;
// }

fn dfs_iter(nums: &[i32], mut i: usize, visited: &mut Vec<bool>) -> i32 {
    let mut result = 0;
    loop {
        if visited[i] {
            break;
        }
        visited[i] = true;
        i = nums[i] as usize;
        result += 1;
    }
    result
}

#[test]
fn test_array_nesting() {
    let tests = vec![(vec![5, 4, 0, 3, 1, 6, 2], 4)];
    for (nums, want) in tests {
        assert_eq!(array_nesting(nums), want);
    }
}
