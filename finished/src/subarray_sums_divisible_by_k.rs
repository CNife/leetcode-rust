pub fn subarrays_div_by_k(array: Vec<i32>, k: i32) -> i32 {
    let mut mod_map = vec![0; k as usize];
    mod_map[0] = 1;

    let mut sum = 0;
    let mut result = 0;
    for num in array {
        sum = (sum + num) % k;
        if sum < 0 {
            sum += k;
        }

        let count = mod_map.get_mut(sum as usize).unwrap();
        if *count > 0 {
            result += *count;
        }
        *count += 1;
    }
    result
}

#[test]
fn test() {
    let cases = vec![(vec![4, 5, 0, -2, -3, 1], 5, 7), (vec![-1, 2, 9], 2, 2)];
    for (array, k, expect) in cases {
        assert_eq!(subarrays_div_by_k(array, k), expect);
    }
}
