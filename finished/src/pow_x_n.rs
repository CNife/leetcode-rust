pub fn my_pow(x: f64, n: i32) -> f64 {
    if x == 0.0 {
        return 0.0;
    }
    if (x - 1.0).abs() < f64::EPSILON || n == 0 {
        return 1.0;
    }
    if (x + 1.0).abs() < f64::EPSILON {
        return if (n & 1) == 0 { 1.0 } else { -1.0 };
    }
    if x.is_infinite() {
        return if x > 0.0 {
            if n > 0 {
                f64::INFINITY
            } else {
                0.0
            }
        } else if n > 0 {
            f64::NEG_INFINITY
        } else {
            0.0
        };
    }
    if x.is_nan() {
        return f64::NAN;
    }

    pow_regular(x, n)
}

fn pow_regular(x: f64, n: i32) -> f64 {
    debug_assert!(
        x.is_finite()
            && x != 0.0
            && (x - 1.0).abs() > f64::EPSILON
            && (x + 1.0).abs() > f64::EPSILON
    );
    debug_assert!(n != 0);

    let (x, n) = if n > 0 {
        (x, n)
    } else if n == i32::MIN {
        (x * x, i32::MIN / 2)
    } else {
        (1.0 / x, -n)
    };
    if n == 1 {
        x
    } else if (n & 1) == 0 {
        pow_regular(x * x, n / 2)
    } else {
        x * pow_regular(x, n - 1)
    }
}

#[test]
fn test() {
    let cases = vec![(2.0, 10, 1024.0), (2.1, 3, 9.261), (2.0, -2, 0.25)];
    for (x, n, expect) in cases {
        assert!(my_pow(x, n) - expect < 1e-6);
    }
}
