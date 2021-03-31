use std::cmp::min;

pub fn shortest_to_char<S: AsRef<str>>(s: S, c: char) -> Vec<i32> {
    let s = s.as_ref();

    let l_dist_vec = distance_vec(s.chars(), c);
    let mut r_dist_vec = distance_vec(s.chars().rev(), c);
    r_dist_vec.reverse();
    let mut res = Vec::with_capacity(s.len());

    for pair in l_dist_vec.into_iter().zip(r_dist_vec.into_iter()) {
        let res_dist = match pair {
            (Some(l), None) => l,
            (None, Some(r)) => r,
            (Some(l), Some(r)) => min(l, r),
            (None, None) => unreachable!(),
        };
        res.push(res_dist);
    }
    res
}

fn distance_vec<I>(itr: I, expected: char) -> Vec<Option<i32>>
where
    I: Iterator<Item = char>,
{
    let mut res = Vec::new();
    let mut prev_idx: Option<usize> = None;

    for (idx, ch) in itr.enumerate() {
        let distance = if ch == expected {
            prev_idx = Some(idx);
            Some(0)
        } else if let Some(prev) = prev_idx {
            Some((idx - prev) as i32)
        } else {
            None
        };
        res.push(distance);
    }
    res
}

#[test]
fn test_shortest_to_char() {
    let cases = vec![(
        "loveleetcode",
        'e',
        vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0],
    )];
    for (s, c, expected) in cases {
        let output = shortest_to_char(s, c);
        assert_eq!(output, expected);
    }
}
