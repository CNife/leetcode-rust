use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn largest_number(nums: Vec<i32>) -> String {
    if nums.is_empty() {
        return "".into();
    }

    let mut heap: BinaryHeap<Str> = nums.into_iter().map(|num| Str(num.to_string())).collect();
    if heap.peek().unwrap().0.parse::<i32>().unwrap() == 0 {
        return "0".into();
    }

    let mut result = String::with_capacity(heap.iter().map(|num_string| num_string.0.len()).sum());
    while let Some(num_string) = heap.pop() {
        result.push_str(&num_string.0);
    }
    result
}

struct Str(String);

impl Ord for Str {
    fn cmp(&self, other: &Self) -> Ordering {
        Iterator::zip(
            self.0.bytes().chain(other.0.bytes()),
            other.0.bytes().chain(self.0.bytes()),
        )
        .find(|(lhs, rhs)| lhs != rhs)
        .map_or(Ordering::Equal, |(lhs, rhs)| lhs.cmp(&rhs))
    }
}

impl PartialOrd for Str {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Str {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Str {}

#[test]
fn test() {
    let cases = vec![
        (vec![10, 2], "210"),
        (vec![3, 30, 34, 5, 9], "9534330"),
        (vec![0, 0, 0], "0"),
    ];
    for (nums, expect) in cases {
        assert_eq!(largest_number(nums), expect);
    }
}
