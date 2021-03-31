pub fn can_partition(nums: Vec<i32>) -> bool {
    let sum: i32 = nums.iter().cloned().sum();
    if sum & 1 != 0 {
        return false;
    }

    let target = (sum / 2) as usize;
    let mut dp = vec![false; target + 1];
    dp[0] = true;

    let first_elem = nums[0] as usize;
    if first_elem <= target {
        dp[nums[0] as usize] = true;
    }

    for elem in nums[1..].iter().map(|elem| *elem as usize) {
        if dp[target] {
            return true;
        }
        for i in (elem..=target).rev() {
            dp[i] = dp[i] || dp[i - elem];
        }
    }
    dp[target]
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 5, 11, 5], true),
        (vec![1, 2, 3, 5], false),
        (vec![100], false),
    ];
    for (nums, expected) in cases {
        assert_eq!(can_partition(nums), expected);
    }
}
