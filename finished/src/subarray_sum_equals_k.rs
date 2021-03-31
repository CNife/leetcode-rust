// pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
//     let prefix_sum = prefix_sum(nums);
//     let mut result = 0;
//     for i in 0..prefix_sum.len() - 1 {
//         for j in i + 1..prefix_sum.len() {
//             if prefix_sum[j] - prefix_sum[i] == k {
//                 result += 1;
//             }
//         }
//     }
//     result
// }
//
// fn prefix_sum(nums: Vec<i32>) -> Vec<i32> {
//     let mut result = Vec::with_capacity(nums.len() + 1);
//     result.push(0);
//
//     let mut sum = 0;
//     for num in nums {
//         sum += num;
//         result.push(sum);
//     }
//     result
// }

use std::collections::HashMap;

pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(0, 1);

    let mut sum = 0;
    let mut result = 0;
    for num in nums {
        sum += num;
        if let Some(count) = map.get(&(sum - k)) {
            result += *count;
        }
        map.entry(sum).and_modify(|count| *count += 1).or_insert(1);
    }
    result
}

#[test]
fn test() {
    let cases = vec![(vec![1, 1, 1], 2, 2)];
    for (nums, k, expect) in cases {
        assert_eq!(subarray_sum(nums, k), expect);
    }
}
