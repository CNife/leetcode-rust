pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
    is_overlap(rec1[0], rec1[2], rec2[0], rec2[2]) && is_overlap(rec1[1], rec1[3], rec2[1], rec2[3])
}

fn is_overlap(l1: i32, r1: i32, l2: i32, r2: i32) -> bool {
    if l1 <= l2 {
        r1 > l2
    } else {
        r2 > l1
    }
}

#[test]
fn test() {
    let cases = vec![
        // (vec![0, 0, 2, 2], vec![1, 1, 3, 3], true),
        (vec![0, 0, 1, 1], vec![1, 0, 2, 1], false),
    ];
    for (rec1, rec2, expected) in cases {
        assert_eq!(is_rectangle_overlap(rec1, rec2), expected);
    }
}
