use std::cmp::Ordering::*;

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut low = 0;
    let mut high = nums.len();
    while low < high {
        let middle = low + (high - low) / 2;
        match nums[middle].cmp(&target) {
            Equal => return middle as i32,
            Less => low = middle + 1,
            Greater => high = middle,
        }
    }
    -1
}

#[test]
fn test() {
    let cases = vec![
        (vec![-1, 0, 3, 5, 9, 12], 9, 4),
        (vec![-1, 0, 3, 5, 9, 12], 2, -1),
        (vec![5], -5, -1),
        (vec![], 0, -1),
    ];
    for (nums, target, expect) in cases {
        assert_eq!(search(nums, target), expect);
    }
}
