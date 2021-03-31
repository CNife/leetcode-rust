use std::collections::HashMap;

pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
    let mut count_map = HashMap::new();
    for num in deck {
        let count = count_map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut iter = count_map.values();
    match iter.next() {
        Some(&first) if first >= 2 => {
            iter.fold(first, |curr_gcd, &count| gcd(curr_gcd, count)) >= 2
        }
        _ => false,
    }
}

fn gcd(lhs: i32, rhs: i32) -> i32 {
    if rhs == 0 {
        lhs
    } else {
        gcd(rhs, lhs % rhs)
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 3, 4, 4, 3, 2, 1], true),
        (vec![1, 1, 1, 2, 2, 2, 3, 3], false),
        (vec![1], false),
        (vec![1, 1, 2, 2, 2, 2], true),
    ];
    for (deck, expect) in cases {
        assert_eq!(has_groups_size_x(deck), expect);
    }
}
