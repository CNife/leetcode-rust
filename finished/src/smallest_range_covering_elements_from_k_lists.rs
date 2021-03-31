use std::cmp::{max, Reverse};
use std::collections::BinaryHeap;

pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let mut left = -100001;
    let mut right = 100001;
    let mut max_value = nums.iter().map(|n| n[0]).max().unwrap();
    let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = nums
        .iter()
        .enumerate()
        .map(|(i, n)| Reverse((n[0], i, 0)))
        .collect();

    while let Some(Reverse((min_value, row, idx))) = heap.pop() {
        if max_value - min_value < right - left {
            left = min_value;
            right = max_value;
        }
        if idx == nums[row].len() - 1 {
            break;
        }
        max_value = max(max_value, nums[row][idx + 1]);
        heap.push(Reverse((nums[row][idx + 1], row, idx + 1)));
    }
    vec![left, right]
}

#[test]
fn test() {
    let tests = vec![(
        vec![
            vec![4, 10, 15, 24, 26],
            vec![0, 9, 12, 20],
            vec![5, 18, 22, 30],
        ],
        vec![20, 24],
    )];
    for (nums, want) in tests {
        assert_eq!(smallest_range(nums), want);
    }
}
