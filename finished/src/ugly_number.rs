use std::ops;

pub fn is_ugly(mut n: i32) -> bool {
    if n < 1 {
        return false;
    }

    macro_rules! divide {
        ($num: expr) => {
            while n > 1 {
                let (div, rem) = dbg!(div_rem(dbg!(n), $num));
                if rem == 0 {
                    n = div;
                } else {
                    break;
                }
            }
        };
    }

    divide!(2);
    divide!(3);
    divide!(5);
    n == 1
}

fn div_rem<T>(x: T, y: T) -> (T, T)
where
    T: ops::Div<Output = T> + ops::Rem<Output = T> + Copy,
{
    (x / y, x % y)
}

#[test]
fn test() {
    assert_eq!(is_ugly(6), true);
    assert_eq!(is_ugly(8), true);
    assert_eq!(is_ugly(14), false);
    assert_eq!(is_ugly(1), true);
}
