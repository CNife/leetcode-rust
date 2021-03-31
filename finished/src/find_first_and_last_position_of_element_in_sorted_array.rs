pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.is_empty() {
        return vec![-1, -1];
    }

    let less_than_target = count_of_less_than(&nums, target);
    let equal_or_less_than_target = count_of_less_than(&nums, target + 1);
    if less_than_target == equal_or_less_than_target {
        vec![-1, -1]
    } else {
        vec![
            less_than_target as i32,
            equal_or_less_than_target as i32 - 1,
        ]
    }
}

// 在从小到大排序的、非空的 nums 中，比 target 小（不计相等）的元素有多少个？
fn count_of_less_than(nums: &[i32], target: i32) -> usize {
    debug_assert!(!nums.is_empty());

    let len = nums.len();
    if nums[0] >= target {
        0
    } else if nums[len - 1] < target {
        len
    } else {
        binary_search_less_than(nums, target)
    }
}

fn binary_search_less_than(nums: &[i32], target: i32) -> usize {
    if nums.is_empty() {
        return 0;
    }

    let mid = nums.len() / 2;
    if nums[mid] < target {
        binary_search_less_than(&nums[mid + 1..], target) + mid + 1
    } else {
        binary_search_less_than(&nums[..mid], target)
    }
}

#[test]
fn test_find_left_bound() {
    let cases = vec![
        (vec![1, 2, 3], 3, 2),
        (vec![1, 2, 3], 4, 3),
        (vec![1], 1, 0),
    ];
    for (nums, target, expected) in cases {
        assert_eq!(count_of_less_than(&nums, target), expected);
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![5, 7, 7, 8, 8, 10], 8, vec![3, 4]),
        (vec![5, 7, 7, 8, 8, 10], 6, vec![-1, -1]),
    ];
    for (nums, target, expected) in cases {
        assert_eq!(search_range(nums, target), expected);
    }
}
