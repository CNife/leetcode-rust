use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for (i, num) in nums.into_iter().enumerate() {
        let another = target - num;
        match map.get(&another) {
            Some(prev_i) => return vec![*prev_i, i as i32],
            None => {
                map.insert(num, i as i32);
            }
        }
    }
    vec![]
}

#[test]
fn test() {
    let cases = vec![(vec![2, 7, 11, 5], 9, vec![0, 1])];
    for (nums, target, expect) in cases {
        assert_eq!(two_sum(nums, target), expect);
    }
}
