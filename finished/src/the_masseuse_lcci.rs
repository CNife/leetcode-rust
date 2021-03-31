use std::cmp::max;

pub fn massage(nums: Vec<i32>) -> i32 {
    let mut not_pick = 0;
    let mut pick = 0;
    for num in nums {
        let old_pick = pick;
        pick = not_pick + num;
        not_pick = max(not_pick, old_pick);
    }
    max(not_pick, pick)
}

// O(N) 空间复杂度的动态规划
//
// use std::cmp::max;
//
// pub fn massage(nums: Vec<i32>) -> i32 {
//     if nums.is_empty() {
//         return 0;
//     }
//
//     let mut dp = vec![[0, 0]; nums.len()];
//     dp[0][1] = nums[0];
//     for i in 1..nums.len() {
//         dp[i][0] = max(dp[i - 1][0], dp[i - 1][1]);
//         dp[i][1] = dp[i - 1][0] + nums[i];
//     }
//     max(dp[nums.len() - 1][0], dp[nums.len() - 1][1])
// }

#[test]
fn test() {
    let cases = vec![
        (vec![], 0),
        (vec![1, 2, 3, 1], 4),
        (vec![2, 7, 9, 3, 1], 12),
        (vec![2, 1, 4, 5, 3, 1, 1, 3], 12),
    ];
    for (nums, expected) in cases {
        assert_eq!(massage(nums), expected);
    }
}
