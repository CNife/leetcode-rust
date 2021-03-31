pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut result = Vec::with_capacity(n);
    let mut product = 1;
    for i in 0..n {
        result.push(product);
        product *= nums[i];
    }

    product = 1;
    for i in (0..n).rev() {
        result[i] *= product;
        product *= nums[i];
    }
    result
}

#[test]
fn test() {
    let cases = vec![(vec![1, 2, 3, 4], vec![24, 12, 8, 6])];
    for (nums, expect) in cases {
        assert_eq!(product_except_self(nums), expect);
    }
}
