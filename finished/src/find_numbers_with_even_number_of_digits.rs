pub fn find_numbers(nums: Vec<i32>) -> i32 {
    let digit_number = |mut n: i32| -> i32 {
        let mut i = 0;
        while n != 0 {
            n /= 10;
            i += 1;
        }
        i
    };

    nums.into_iter()
        .filter(|num| digit_number(*num) % 2 == 0)
        .count() as i32
}

#[test]
fn test() {
    let cases = vec![
        (vec![12, 345, 2, 6, 7896], 2),
        (vec![555, 901, 482, 1771], 1),
    ];
    for (nums, expected) in cases {
        assert_eq!(find_numbers(nums), expected);
    }
}
