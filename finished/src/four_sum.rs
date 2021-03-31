use std::cmp::Ordering::*;
use std::collections::HashSet;

pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    if nums.len() < 4 {
        return vec![];
    }
    nums.sort_unstable();

    let mut result = HashSet::new();
    for i in 0..nums.len() - 3 {
        for j in i + 1..nums.len() - 2 {
            let mut left = j + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = nums[i] + nums[j] + nums[left] + nums[right];
                match sum.cmp(&target) {
                    Equal => {
                        result.insert(vec![nums[i], nums[j], nums[left], nums[right]]);
                        left += 1;
                        right -= 1;
                    }
                    Less => left += 1,
                    Greater => right -= 1,
                }
            }
        }
    }

    result.into_iter().collect()
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![1, 0, -1, 0, -2, 2],
            0,
            vec![vec![-1, 0, 0, 1], vec![-2, -1, 1, 2], vec![-2, 0, 0, 2]],
        ),
        (
            vec![-3, -2, -1, 0, 0, 1, 2, 3],
            0,
            vec![
                vec![-3, -2, 2, 3],
                vec![-3, -1, 1, 3],
                vec![-3, 0, 0, 3],
                vec![-3, 0, 1, 2],
                vec![-2, -1, 0, 3],
                vec![-2, -1, 1, 2],
                vec![-2, 0, 0, 2],
                vec![-1, 0, 0, 1],
            ],
        ),
    ];

    let trans = |v: Vec<Vec<i32>>| -> HashSet<Vec<i32>> {
        v.into_iter()
            .map(|mut inner| {
                inner.sort_unstable();
                inner
            })
            .collect()
    };

    for (nums, target, expected) in cases {
        let output = four_sum(nums, target);
        assert_eq!(output.len(), expected.len());
        assert_eq!(trans(output), trans(expected));
    }
}
