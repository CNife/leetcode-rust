//use std::cmp::max;
//
//pub fn length_of_lis(nums: Vec<i32>) -> i32 {
//    let n = nums.len();
//    if n == 0 {
//        return 0;
//    }
//
//    let mut dp = vec![1; n];
//    for i in 0..n {
//        for j in 0..i {
//            if nums[j] < nums[i] {
//                dp[i] = max(dp[i], dp[j] + 1);
//            }
//        }
//    }
//    dp.into_iter().max().unwrap()
//}

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut dp = Vec::new();
    for num in nums {
        if let Err(index) = dp.binary_search(&num) {
            if index == dp.len() {
                dp.push(num);
            } else {
                dp[index] = num;
            }
        }
    }
    dp.len() as i32
}

#[test]
fn test() {
    let cases = vec![
        (vec![10, 9, 2, 5, 3, 7, 101, 18], 4),
        (vec![1, 3, 6, 7, 9, 4, 10, 5, 6], 6),
    ];
    for (nums, expected) in cases {
        assert_eq!(length_of_lis(nums), expected);
    }
}
