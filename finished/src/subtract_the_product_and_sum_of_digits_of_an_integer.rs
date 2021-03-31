pub fn subtract_product_and_sum(mut n: i32) -> i32 {
    let mut digits = Vec::with_capacity(10);
    while n != 0 {
        digits.push(n % 10);
        n /= 10;
    }
    let product: i32 = digits.iter().product();
    let sum: i32 = digits.into_iter().sum();
    product - sum
}

#[test]
fn test() {
    let cases = vec![(234, 15), (4421, 21)];
    for (n, expected) in cases {
        assert_eq!(subtract_product_and_sum(n), expected);
    }
}
