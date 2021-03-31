pub fn search(nums: Vec<i32>, target: i32) -> bool {
    recursively_search(&nums, target)
}

fn recursively_search(nums: &[i32], target: i32) -> bool {
    if nums.is_empty() {
        return false;
    }

    let mid_idx = nums.len() / 2;
    let start = nums[0];
    let mid = nums[mid_idx];

    if target == start || target == mid {
        true
    } else if start == mid {
        recursively_search(&nums[1..], target)
    } else if start < mid {
        if target > start && target < mid {
            recursively_search(&nums[..mid_idx], target)
        } else {
            recursively_search(&nums[mid_idx..], target)
        }
    } else if target > mid && target < start {
        recursively_search(&nums[mid_idx..], target)
    } else {
        recursively_search(&nums[..mid_idx], target)
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![2, 5, 6, 0, 0, 1, 2], 0, true),
        (vec![2, 5, 6, 0, 0, 1, 2], 3, false),
    ];
    for (nums, target, expected) in cases {
        assert_eq!(search(nums, target), expected);
    }
}
