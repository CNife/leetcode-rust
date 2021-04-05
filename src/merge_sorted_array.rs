pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut i = m - 1;
    let mut j = n - 1;
    let mut k = m + n - 1;
    while i >= 0 && j >= 0 {
        nums1[k as usize] = if nums1[i as usize] > nums2[j as usize] {
            i -= 1;
            nums1[(i + 1) as usize]
        } else {
            j -= 1;
            nums2[(j + 1) as usize]
        };
        k -= 1;
    }
    while j >= 0 {
        nums1[k as usize] = nums2[j as usize];
        j -= 1;
        k -= 1;
    }
}

#[test]
fn test() {
    let tests = vec![
        (vec![1, 2, 3, 0, 0, 0], 3, vec![2, 5, 6], 3, vec![1, 2, 2, 3, 5, 6]),
        (vec![1], 1, vec![], 0, vec![1]),
    ];
    for (mut nums1, m, mut nums2, n, want) in tests {
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, want);
    }
}
