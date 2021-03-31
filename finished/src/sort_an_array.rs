pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable();
    nums
}

#[test]
fn test() {
    let cases = vec![vec![5, 2, 3, 1], vec![5, 1, 1, 2, 0, 0]];
    for nums in cases {
        let actual = sort_array(nums.clone());
        let mut expect = nums.clone();
        expect.sort_unstable();
        assert_eq!(actual, expect, "{:?}", nums);
    }
}
