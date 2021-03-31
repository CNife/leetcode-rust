pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut candidate = 0;
    let mut count = 0;
    for num in nums {
        if count <= 0 {
            candidate = num;
        }
        count += if num == candidate { 1 } else { -1 };
    }
    candidate
}

#[test]
fn test() {
    let cases = vec![(vec![3, 2, 3], 3), (vec![2, 2, 1, 1, 1, 2, 2], 2)];
    for (nums, expected) in cases {
        assert_eq!(majority_element(nums), expected);
    }
}
