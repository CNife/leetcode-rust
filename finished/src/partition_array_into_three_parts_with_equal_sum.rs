pub fn can_three_parts_equal_sum(nums: Vec<i32>) -> bool {
    if nums.len() < 3 {
        return false;
    }

    let sum: i32 = nums.iter().sum();
    let target = sum / 3;
    if target * 3 != sum {
        return false;
    }

    let mut sum = 0;
    let mut i = 0;
    for num in nums.iter() {
        sum += *num;
        i += 1;
        if sum == target {
            break;
        }
    }
    if i >= nums.len() - 1 {
        return false;
    }

    let mut j = nums.len() - 1;
    sum = 0;
    for num in nums.iter().rev() {
        sum += *num;
        if sum == target {
            break;
        }
        j -= 1;
    }
    if j < i + 1 {
        return false;
    }
    nums[i..j].iter().sum::<i32>() == target
}

#[test]
fn test() {
    let cases = vec![
        (vec![0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1], true),
        (vec![0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1], false),
        (vec![3, 3, 6, 5, -2, 2, 5, 1, -9, 4], true),
        (vec![12, -4, 16, -5, 9, -3, 3, 8, 0], true),
        (vec![14, 6, -10, 2, 18, -7, -4, 11], false),
        (vec![18, 12, -18, 18, -19, -1, 10, 10], true),
    ];
    for (nums, expected) in cases {
        assert_eq!(
            can_three_parts_equal_sum(nums.clone()),
            expected,
            "Test case {:?}",
            nums
        );
    }
}
