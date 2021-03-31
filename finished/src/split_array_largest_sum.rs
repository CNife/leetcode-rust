pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
    let mut low = *nums.iter().max().unwrap();
    let mut high = nums.iter().sum::<i32>();
    while low < high {
        let middle = low + (high - low) / 2;
        let mut count = 0;
        let mut sub_sum = 0;
        for &num in &nums {
            sub_sum += num;
            if sub_sum > middle {
                count += 1;
                sub_sum = num;
            }
        }
        count += 1;
        if count > m {
            low = middle + 1;
        } else {
            high = middle;
        }
    }
    low
}

#[test]
fn test() {
    let tests = vec![(vec![7, 2, 5, 10, 8], 2, 18)];
    for (nums, m, want) in tests {
        assert_eq!(split_array(nums, m), want);
    }
}
