pub fn find_relative_ranks(nums: Vec<i32>) -> Vec<String> {
    let mut nums_with_index: Vec<(usize, i32)> = nums.into_iter().enumerate().collect();
    nums_with_index.sort_unstable_by(|(_, lhs), (_, rhs)| i32::cmp(rhs, lhs));

    let mut ranking = vec![0usize; nums_with_index.len()];
    for (i, (index, _)) in nums_with_index.into_iter().enumerate() {
        ranking[index] = i + 1;
    }

    ranking
        .into_iter()
        .map(|n| match n {
            1 => "Gold Medal".to_string(),
            2 => "Silver Medal".to_string(),
            3 => "Bronze Medal".to_string(),
            n => n.to_string(),
        })
        .collect()
}

#[test]
fn test() {
    let cases = vec![(
        vec![5, 4, 3, 2, 1],
        vec!["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"],
    )];
    for (nums, expected) in cases {
        assert_eq!(find_relative_ranks(nums), expected);
    }
}
