use std::collections::{BinaryHeap, HashMap};

pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        nums.len() as i32
    } else {
        keys_with_max_values(count_map(&nums))
            .into_iter()
            .map(|key| subarray_length_of_key(key, &nums))
            .min()
            .unwrap() as i32
    }
}

fn count_map(nums: &[i32]) -> HashMap<i32, i32> {
    let mut count_map = HashMap::new();
    for &num in nums {
        *count_map.entry(num).or_insert(0) += 1;
    }
    count_map
}

fn keys_with_max_values(counts: HashMap<i32, i32>) -> Vec<i32> {
    let mut count_heap: BinaryHeap<(i32, i32)> = counts
        .into_iter()
        .map(|(key, count)| (count, key))
        .collect();

    debug_assert!(!count_heap.is_empty());
    let max_count = count_heap.peek().unwrap().0;
    let mut result = Vec::new();
    while let Some((count, key)) = count_heap.pop() {
        if count == max_count {
            result.push(key);
        } else {
            break;
        }
    }
    result
}

fn subarray_length_of_key(key: i32, nums: &[i32]) -> usize {
    let left = nums
        .iter()
        .enumerate()
        .find(|(_, num)| **num == key)
        .unwrap()
        .0;
    let right = nums
        .iter()
        .enumerate()
        .rfind(|(_, num)| **num == key)
        .unwrap()
        .0;
    right - left + 1
}

#[test]
fn test() {
    let cases = vec![(vec![1, 2, 2, 3, 1], 2), (vec![1, 2, 2, 3, 1, 4, 2], 6)];
    for (nums, expect) in cases {
        assert_eq!(find_shortest_sub_array(nums), expect);
    }
}
