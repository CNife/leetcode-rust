use std::cmp::{max, min};
use std::i32;

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let m = nums1.len();
    let n = nums2.len();

    if m > n {
        find_median_sorted_arrays(nums2, nums1)
    } else {
        let mut lo = 0;
        let mut hi = 2 * m;
        let mut l_max1 = i32::MAX;
        let mut l_max2 = i32::MAX;
        let mut r_min1 = i32::MAX;
        let mut r_min2 = i32::MAX;
        while lo <= hi {
            let c1 = (lo + hi) / 2;
            let c2 = m + n - c1;

            l_max1 = if c1 == 0 {
                i32::MIN
            } else {
                nums1[(c1 - 1) / 2]
            };
            l_max2 = if c2 == 0 {
                i32::MIN
            } else {
                nums2[(c2 - 1) / 2]
            };
            r_min1 = if c1 == 2 * m { i32::MAX } else { nums1[c1 / 2] };
            r_min2 = if c2 == 2 * n { i32::MAX } else { nums2[c2 / 2] };

            if l_max1 > r_min2 {
                hi = c1 - 1;
            } else if l_max2 > r_min1 {
                lo = c1 + 1;
            } else {
                break;
            }
        }
        (max(l_max1, l_max2) + min(r_min1, r_min2)) as f64 / 2.0
    }
}

#[test]
fn test_find_median_sorted_arrays() {
    let cases = vec![
        (vec![1, 3], vec![2], 2.0),
        (vec![1, 2], vec![3, 4], 2.5),
        (vec![3, 4], vec![1, 2, 5], 3.0),
    ];
    for (nums1, nums2, expected) in cases {
        let output = find_median_sorted_arrays(nums1, nums2);
        assert_eq!(output, expected);
    }
}
