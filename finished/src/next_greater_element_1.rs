use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let map: HashMap<i32, usize> = nums2.iter().enumerate().map(|(i, n)| (*n, i)).collect();
        nums1
            .into_iter()
            .map(|n1| *nums2[map[&n1]..].iter().find(|n2| **n2 > n1).unwrap_or(&-1))
            .collect()
    }
}

#[test]
fn test_next_greater_element() {
    let cases = vec![
        (vec![4, 1, 2], vec![1, 3, 4, 2], vec![-1, 3, -1]),
        (vec![2, 4], vec![1, 2, 3, 4], vec![3, -1]),
    ];
    for (nums1, nums2, expected) in cases {
        let output = Solution::next_greater_element(nums1, nums2);
        assert_eq!(output, expected);
    }
}
