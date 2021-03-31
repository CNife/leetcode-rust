use std::cmp::Ordering::*;

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let nums = nums;

    let mut res = Vec::new();
    if nums.len() < 3 || nums[0] > 0 || nums[nums.len() - 1] < 0 {
        return res;
    }

    let mut i = 0;
    while i < nums.len() - 2 {
        if nums[i] > 0 {
            break;
        }
        let mut j = i + 1;
        let mut k = nums.len() - 1;
        while j < k {
            let sum = nums[i] + nums[j] + nums[k];
            match sum.cmp(&0) {
                Equal => {
                    res.push(vec![nums[i], nums[j], nums[k]]);
                    j = next_right_different(&nums, j);
                    k = next_left_different(&nums, k);
                }
                Less => j = next_right_different(&nums, j),
                Greater => k = next_left_different(&nums, k),
            }
        }
        i = next_right_different(&nums, i);
    }
    res
}

#[inline]
fn next_left_different(nums: &[i32], mut idx: usize) -> usize {
    let before = nums[idx];
    loop {
        idx -= 1;
        if idx == 0 || nums[idx] != before {
            break;
        }
    }
    idx
}

#[inline]
fn next_right_different(nums: &[i32], mut idx: usize) -> usize {
    let before = nums[idx];
    loop {
        idx += 1;
        if idx >= nums.len() || nums[idx] != before {
            break;
        }
    }
    idx
}

#[test]
fn test_three_sum() {
    use utils::assert_same_set;

    let cases = vec![
        (
            vec![-1, 0, 1, 2, -1, -4],
            vec![vec![-1, 0, 1], vec![-1, -1, 2]],
        ),
        (vec![], vec![]),
        (vec![0, 0, 0], vec![vec![0, 0, 0]]),
    ];
    for (nums, expected) in cases {
        let output = three_sum(nums);
        assert_same_set(output, expected);
    }
}
