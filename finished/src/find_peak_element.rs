pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    match nums.len() {
        0 => -1,
        1 => 0,
        n => {
            let mut i = 0;
            let mut j = n;
            while i < j - 1 {
                let mid = (i + j) / 2;
                if nums[mid - 1] > nums[mid] {
                    j = mid;
                } else {
                    i = mid;
                }
            }
            i as i32
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 3, 1], vec![2]),
        (vec![1, 2, 1, 3, 5, 6, 4], vec![1, 5]),
    ];
    for (nums, possible_values) in cases {
        let output = find_peak_element(nums);
        assert!(possible_values.into_iter().any(|value| value == output));
    }
}
