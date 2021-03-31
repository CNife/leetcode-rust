pub fn smallest_range_i(arr: Vec<i32>, k: i32) -> i32 {
    let mut max = arr[0];
    let mut min = max;
    for n in arr {
        if n > max {
            max = n;
        } else if n < min {
            min = n;
        }
    }
    if max - k <= min + k {
        0
    } else {
        (max - k) - (min + k)
    }
}

#[test]
fn test_smallest_range_i() {
    let cases = vec![(vec![1], 0, 0), (vec![0, 10], 2, 6), (vec![1, 3, 6], 3, 0)];
    for (arr, k, expected) in cases {
        let output = smallest_range_i(arr, k);
        assert_eq!(output, expected);
    }
}
