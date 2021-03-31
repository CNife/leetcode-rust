pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let n = nums.len();
    if n < 3 {
        return n as i32;
    }

    let mut left = 1;
    let mut count = 1;

    for right in 1..n {
        if nums[right] == nums[right - 1] {
            count += 1;
        } else {
            count = 1;
        }
        if count <= 2 {
            nums[left] = nums[right];
            left += 1;
        }
    }

    left as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let cases = vec![
            (vec![1, 1, 1, 2, 2, 3], 5),
            (vec![0, 0, 1, 1, 1, 1, 2, 3, 3], 7),
        ];
        for (mut nums, expected_len) in cases {
            assert_eq!(remove_duplicates(&mut nums), expected_len as i32);
            check_no_dup(&nums[..expected_len]);
        }
    }

    fn check_no_dup(nums: &[i32]) {
        let mut dup = false;
        for i in 0..nums.len() - 1 {
            assert!(!dup || nums[i] != nums[i + 1]);
            dup = nums[i] == nums[i + 1];
        }
    }
}
