pub fn decompress_rle_list(nums: Vec<i32>) -> Vec<i32> {
    debug_assert_eq!(nums.len() % 2, 0);

    let mut i = 0;
    let mut result = Vec::new();
    while i < nums.len() {
        let count = nums[i];
        let num = nums[i + 1];
        for _ in 0..count {
            result.push(num);
        }
        i += 2;
    }
    result
}

#[test]
fn test() {
    let cases = vec![(vec![1, 2, 3, 4], vec![2, 4, 4, 4])];
    for (nums, expected) in cases {
        assert_eq!(decompress_rle_list(nums), expected);
    }
}
