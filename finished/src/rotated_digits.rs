use Number::*;

#[derive(Copy, Clone)]
enum Number {
    Good,
    Bad,
    Common,
}

pub fn rotated_digits(n: i32) -> i32 {
    let n = n as usize;
    let mut arr = vec![
        Common, Common, Good, Bad, Bad, Good, Good, Bad, Common, Good,
    ];
    if n > arr.len() {
        arr.reserve(n - arr.len());
    }

    let mut res = 0;
    for i in 1..=n {
        let number = match (arr[i / 10], arr[i % 10]) {
            (Bad, _) | (_, Bad) => Bad,
            (Common, Common) => Common,
            _ => Good,
        };
        if i > 9 {
            arr.push(number);
        }
        if let Good = number {
            res += 1;
        }
    }
    res
}

#[test]
fn test() {
    assert_eq!(std::mem::size_of::<Number>(), 1);
    assert_eq!(rotated_digits(10), 4);
    assert_eq!(rotated_digits(2), 1);
}
