use std::cmp::Ordering;

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut i = 0usize;
    let mut j = numbers.len() - 1;
    while i < j {
        let sum = numbers[i] + numbers[j];
        match sum.cmp(&target) {
            Ordering::Equal => return vec![i as i32 + 1, j as i32 + 1],
            Ordering::Less => i += 1,
            Ordering::Greater => j -= 1,
        }
    }
    unreachable!()
}

#[test]
fn test() {
    let cases = vec![(vec![2, 7, 11, 15], 9, vec![1, 2])];
    for (numbers, target, expect) in cases {
        assert_eq!(two_sum(numbers, target), expect);
    }
}
