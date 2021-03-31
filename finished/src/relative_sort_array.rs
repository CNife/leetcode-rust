use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let base_key = arr2.len();
        let cmp_map: HashMap<_, _> = arr2
            .into_iter()
            .enumerate()
            .map(|(i, elem)| (elem, i))
            .collect();

        arr1.sort_unstable_by_key(|elem| {
            *cmp_map.get(elem).unwrap_or(&(base_key + *elem as usize))
        });
        arr1
    }
}

#[test]
fn test_relative_sort_array() {
    let input_1 = vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19];
    let input_2 = vec![2, 1, 4, 3, 9, 6];
    let expected = vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19];

    let output = Solution::relative_sort_array(input_1, input_2);
    assert_eq!(output, expected);
}
