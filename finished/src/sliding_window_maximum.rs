use std::cmp::max;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    if k < 1 || nums.is_empty() {
        vec![]
    } else if k == 1 {
        nums
    } else if nums.len() <= k as usize {
        vec![nums.into_iter().max().unwrap()]
    } else {
        let n = nums.len();
        let k = k as usize;
        let left = {
            let mut left = Vec::with_capacity(n);
            let mut i = 0;
            let mut block_max = 0;
            for &num in &nums {
                if i >= k {
                    i = 0;
                    block_max = num;
                }
                block_max = max(block_max, num);
                left.push(block_max);
                i += 1;
            }
            left
        };
        let right = {
            let mut right = Vec::with_capacity(n);
            let mut i = n % k;
            let mut block_max = 0;
            for &num in nums.iter().rev() {
                if i == 0 {
                    i = k;
                    block_max = num;
                }
                block_max = max(block_max, num);
                right.push(block_max);
                i -= 1;
            }
            right.reverse();
            right
        };

        let mut result = Vec::with_capacity(n - k);
        for i in 0..=n - k {
            let j = i + k - 1;
            result.push(max(left[j], right[i]));
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![(vec![1, 3, -1, -3, 5, 3, 6, 7], 3, vec![3, 3, 5, 5, 6, 7])];
    for (nums, k, expected) in cases {
        assert_eq!(max_sliding_window(nums, k), expected);
    }
}
