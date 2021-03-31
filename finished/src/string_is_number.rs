#[derive(Debug, Copy, Clone)]
enum Status {
    Init,
    SignRead,
    Integer,
    DotRead,
    Decimal,
    ERead,
    ExponentSignRead,
    ExponentInteger,
    TrailingSpace,
    LeadingDot,
    Error,
}

pub fn is_number(input: String) -> bool {
    use Status::*;

    let mut status = Init;
    for ch in input.chars() {
        status = match status {
            Init => match ch {
                ' ' => Init,
                '+' | '-' => SignRead,
                '.' => LeadingDot,
                ch if ch.is_ascii_digit() => Integer,
                _ => Error,
            },
            SignRead => match ch {
                '.' => LeadingDot,
                ch if ch.is_ascii_digit() => Integer,
                _ => Error,
            },
            Integer => match ch {
                'e' | 'E' => ERead,
                ' ' => TrailingSpace,
                '.' => DotRead,
                ch if ch.is_ascii_digit() => Integer,
                _ => Error,
            },
            DotRead => match ch {
                'e' | 'E' => ERead,
                ' ' => TrailingSpace,
                ch if ch.is_ascii_digit() => Decimal,
                _ => Error,
            },
            Decimal => match ch {
                'e' | 'E' => ERead,
                ' ' => TrailingSpace,
                ch if ch.is_ascii_digit() => Decimal,
                _ => Error,
            },
            LeadingDot => {
                if ch.is_ascii_digit() {
                    Decimal
                } else {
                    Error
                }
            }
            ERead => match ch {
                '+' | '-' => ExponentSignRead,
                ch if ch.is_ascii_digit() => ExponentInteger,
                _ => Error,
            },
            ExponentSignRead => {
                if ch.is_ascii_digit() {
                    ExponentInteger
                } else {
                    Error
                }
            }
            ExponentInteger => match ch {
                ' ' => TrailingSpace,
                ch if ch.is_ascii_digit() => ExponentInteger,
                _ => Error,
            },
            TrailingSpace => {
                if ch == ' ' {
                    TrailingSpace
                } else {
                    Error
                }
            }
            Error => return false,
        };
    }
    match status {
        Integer | DotRead | Decimal | ExponentInteger | TrailingSpace => true,
        _ => false,
    }
}

#[test]
fn test() {
    let tests = vec![
        ("+100", true),
        ("5e2", true),
        ("-123", true),
        ("3.1416", true),
        ("-1E-16", true),
        ("0123", true),
        ("12e", false),
        ("1a3.14", false),
        ("1.2.3", false),
        ("+-5", false),
        ("12e+5.4", false),
    ];
    for (input, want) in tests {
        assert_eq!(is_number(input.into()), want);
    }
}
