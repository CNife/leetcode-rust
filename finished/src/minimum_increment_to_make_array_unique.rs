pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return 0;
    }

    nums.sort_unstable();
    let mut moves = 0;
    for i in 1..nums.len() {
        if nums[i] <= nums[i - 1] {
            let old = nums[i];
            nums[i] = nums[i - 1] + 1;
            moves += nums[i] - old;
        }
    }
    moves
}

#[test]
fn test() {
    let cases = vec![(vec![1, 2, 2], 1), (vec![3, 2, 1, 2, 1, 7], 6)];
    for (nums, expected) in cases {
        assert_eq!(min_increment_for_unique(nums), expected);
    }
}
