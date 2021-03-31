pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    let mut cnt = vec![0; nums.len() + 1];
    cnt[0] = 1;
    let mut odd = 0;
    let mut ans = 0;
    for num in nums {
        odd += num & 1;
        if odd >= k {
            ans += cnt[(odd - k) as usize];
        }
        cnt[odd as usize] += 1;
    }
    ans
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 1, 2, 1, 1], 3, 2),
        (vec![2, 4, 6], 1, 0),
        (vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2, 16),
    ];
    for (nums, k, expect) in cases {
        assert_eq!(number_of_subarrays(nums, k), expect);
    }
}
