use std::cmp::{min, Ordering::*};

pub fn arrays_intersection(arr1: Vec<i32>, arr2: Vec<i32>, arr3: Vec<i32>) -> Vec<i32> {
    two_sorted_arrays_intersection(arr1, two_sorted_arrays_intersection(arr2, arr3))
}

fn two_sorted_arrays_intersection(a1: Vec<i32>, a2: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::with_capacity(min(a1.len(), a2.len()));
    let mut i = 0;
    let mut j = 0;
    while i < a1.len() && j < a2.len() {
        match a1[i].cmp(&a2[j]) {
            Equal => {
                res.push(a1[i]);
                i += 1;
                j += 1;
            }
            Less => i += 1,
            Greater => j += 1,
        }
    }
    res
}

#[test]
fn test_arrays_intersection() {
    let cases = vec![(
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 5, 7, 9],
        vec![1, 3, 4, 5, 8],
        vec![1, 5],
    )];

    for (a1, a2, a3, expected) in cases {
        let output = arrays_intersection(a1, a2, a3);
        assert_eq!(output, expected);
    }
}
