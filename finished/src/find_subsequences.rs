pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = HashSet::new();
        let mut seq = Vec::new();
        Solution::backtrace(&mut res, &mut seq, &nums, 0);
        res.into_iter().collect::<Vec<_>>()
    }

    fn backtrace(res: &mut HashSet<Vec<i32>>, seq: &mut Vec<i32>, nums: &[i32], start: usize) {
        if seq.len() >= 2 {
            res.insert(seq.clone());
        }
        for i in start..nums.len() {
            if *seq.last().unwrap_or(&std::i32::MIN) <= nums[i] {
                seq.push(nums[i]);
                Solution::backtrace(res, seq, nums, i + 1);
                seq.pop();
            }
        }
    }
}

#[test]
fn test_find_subsequences() {
    let input = vec![4, 6, 7, 7];
    let expected = {
        let mut set = HashSet::new();
        set.insert(vec![4, 6]);
        set.insert(vec![4, 7]);
        set.insert(vec![4, 6, 7]);
        set.insert(vec![4, 6, 7, 7]);
        set.insert(vec![6, 7]);
        set.insert(vec![6, 7, 7]);
        set.insert(vec![7, 7]);
        set.insert(vec![4, 7, 7]);
        set
    };

    let output = Solution::find_subsequences(input)
        .into_iter()
        .collect::<HashSet<_>>();
    assert_eq!(output, expected);
}
