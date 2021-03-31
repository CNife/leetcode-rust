use std::cmp::max;
use std::collections::HashMap;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut result = 0;
    for num in nums {
        if map.get(&num).is_none() {
            let left_len = map.get(&(num - 1)).map_or(0, |len| *len);
            let right_len = map.get(&(num + 1)).map_or(0, |len| *len);
            let new_len = left_len + right_len + 1;
            result = max(result, new_len);
            map.insert(num, new_len);
            map.insert(num - left_len, new_len);
            map.insert(num + right_len, new_len);
        }
    }
    result
}

#[test]
fn test() {
    let cases = vec![
        (vec![100, 4, 200, 1, 3, 2], 4),
        (
            vec![
                4, 0, -4, -2, 2, 5, 2, 0, -8, -8, -8, -8, -1, 7, 4, 5, 5, -4, 6, 6, -3,
            ],
            5,
        ),
    ];
    for (nums, expected) in cases {
        assert_eq!(longest_consecutive(nums), expected);
    }
}
