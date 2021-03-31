use std::collections::BinaryHeap;

pub fn get_least_numbers(array: Vec<i32>, k: i32) -> Vec<i32> {
    if k <= 0 {
        return vec![];
    }
    let k = k as usize;
    if array.len() <= k {
        return array;
    }

    let mut heap = BinaryHeap::with_capacity(k);
    for num in array {
        if heap.len() < k {
            heap.push(num);
        } else if num < *heap.peek().unwrap() {
            *heap.peek_mut().unwrap() = num;
        }
    }
    heap.into_vec()
}

#[test]
fn test() {
    use utils::assert_same_set;

    let cases = vec![
        (vec![3, 2, 1], 2, vec![1, 2]),
        (vec![0, 1, 2, 1], 1, vec![0]),
    ];
    for (array, k, expected) in cases {
        assert_same_set(get_least_numbers(array, k), expected);
    }
}
