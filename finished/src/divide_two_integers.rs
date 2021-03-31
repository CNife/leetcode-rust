use std::i32;

pub fn divide(dividend: i32, divisor: i32) -> i32 {
    if let Some(result) = handle_special_cases(dividend, divisor) {
        return result;
    }
    debug_assert!(!vec![0, 1, -1, i32::MIN].contains(&divisor));
    debug_assert!(dividend == i32::MIN || dividend.abs() > divisor.abs());

    let (abs_result, negative_result) = if dividend > 0 && divisor < 0 {
        (recursively_divide(-dividend, divisor), true)
    } else if dividend > 0 && divisor > 0 {
        (recursively_divide(-dividend, -divisor), false)
    } else if dividend < 0 && divisor < 0 {
        (recursively_divide(dividend, divisor), false)
    } else {
        (recursively_divide(dividend, -divisor), true)
    };
    if negative_result {
        -abs_result
    } else {
        abs_result
    }
}

fn handle_special_cases(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        Some(0)
    } else if divisor == 1 {
        Some(dividend)
    } else if divisor == -1 {
        Some(if dividend == i32::MIN {
            i32::MAX
        } else {
            -dividend
        })
    } else if divisor == i32::MIN {
        Some(if dividend == i32::MIN { 1 } else { 0 })
    } else if dividend == i32::MIN {
        None
    } else if dividend == divisor {
        Some(1)
    } else if dividend == -divisor {
        Some(-1)
    } else if dividend.abs() < divisor.abs() {
        Some(0)
    } else {
        None
    }
}

fn recursively_divide(dividend: i32, divisor: i32) -> i32 {
    if dividend > divisor {
        return 0;
    }

    let mut result = 1;
    let mut next_divisor = divisor;
    while let Some(next) = next_divisor.checked_add(next_divisor) {
        if dividend > next {
            break;
        } else {
            result += result;
            if dividend == next {
                return result;
            } else {
                next_divisor = next;
            }
        }
    }
    result + recursively_divide(dividend - next_divisor, divisor)
}

#[test]
fn test() {
    let cases = vec![
        (10, 3, 3),
        (7, -3, -2),
        (i32::MIN, i32::MIN, 1),
        (1, i32::MIN, 0),
        (i32::MIN, i32::MAX, -1),
        (i32::MIN, -1, i32::MAX),
    ];
    for (dividend, divisor, expected) in cases {
        assert_eq!(divide(dividend, divisor), expected);
    }
}
