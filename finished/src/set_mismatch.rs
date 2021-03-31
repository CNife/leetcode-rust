pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let mut exist = vec![false; nums.len()];
    let mut duplicated_num = -1;
    for num in nums {
        let index = num as usize - 1;
        if exist[index] {
            duplicated_num = num;
        } else {
            exist[index] = true;
        }
    }
    let absent_num = 1 + exist
        .into_iter()
        .enumerate()
        .find(|&(_, exists)| !exists)
        .unwrap()
        .0 as i32;

    vec![duplicated_num, absent_num]
}

#[test]
fn test() {
    let cases = vec![(vec![1, 2, 2, 4], vec![2, 3])];
    for (nums, expect) in cases {
        assert_eq!(find_error_nums(nums.clone()), expect, "{:?}", nums);
    }
}
