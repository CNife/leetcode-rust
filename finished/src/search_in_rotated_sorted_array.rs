// 我的解法：递归
//pub fn search(nums: Vec<i32>, target: i32) -> i32 {
//    if nums.is_empty() {
//        -1
//    } else {
//        do_search(&nums, target, 0)
//    }
//}
//
//fn do_search(nums: &[i32], target: i32, offset: usize) -> i32 {
//    let mid_idx = nums.len() / 2;
//    let mid = nums[mid_idx];
//    let start = nums[0];
//
//    if start <= nums[nums.len() - 1] {
//        binary_search(&nums, target, offset)
//    } else if target == mid {
//        (mid_idx + offset) as i32
//    } else if target == start {
//        offset as i32
//    } else if start >= mid {
//        if target < mid || target >= start {
//            do_search(&nums[..mid_idx], target, offset)
//        } else {
//            binary_search(&nums[mid_idx + 1..], target, offset + mid_idx + 1)
//        }
//    } else {
//        if target <= start || target > mid {
//            do_search(&nums[mid_idx + 1..], target, offset + mid_idx + 1)
//        } else {
//            binary_search(&nums[..mid_idx], target, offset)
//        }
//    }
//}
//
//fn binary_search(nums: &[i32], target: i32, offset: usize) -> i32 {
//    match nums.binary_search(&target) {
//        Ok(i) => (i + offset) as i32,
//        Err(_) => -1,
//    }
//}

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.is_empty() {
        return -1;
    }

    let mut l = 0usize;
    let mut h = nums.len() - 1;
    let start = nums[0];
    while l < h {
        let mid = (l + h) / 2;
        if (start > target) ^ (start > nums[mid]) ^ (target > nums[mid]) {
            l = mid + 1;
        } else {
            h = mid;
        }
    }
    if l == h && nums[l] == target {
        l as i32
    } else {
        -1
    }
}

#[test]
fn test_search() {
    let cases = vec![
        (vec![4, 5, 6, 7, 0, 1, 2], 0, 4),
        (vec![4, 5, 6, 7, 0, 1, 2], 3, -1),
        (vec![3, 5, 1], 3, 0),
        (vec![], 1, -1),
    ];
    for (nums, target, expected) in cases {
        let output = search(nums, target);
        assert_eq!(output, expected);
    }
}
