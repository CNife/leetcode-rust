pub fn find_magic_index(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .enumerate()
        .find(|&(index, num)| index == num as usize)
        .map_or(-1, |(_, num)| num)
}

#[test]
fn test() {
    let tests = vec![(vec![0, 2, 3, 4, 5], 0), (vec![1, 1, 1], 1)];
    for (nums, want) in tests {
        assert_eq!(find_magic_index(nums), want);
    }
}
