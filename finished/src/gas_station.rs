pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut total = 0;
    let mut current = 0;
    let mut start = -1;

    for (i, (g, c)) in gas.into_iter().zip(cost.into_iter()).enumerate() {
        current += g - c;
        total += g - c;
        if current < 0 {
            current = 0;
            start = -1;
        } else if start < 0 {
            start = i as i32;
        }
    }

    if total >= 0 {
        start
    } else {
        -1
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2], 3),
        (vec![2, 3, 4], vec![3, 4, 3], -1),
        (vec![5, 1, 2, 3, 4], vec![4, 4, 1, 5, 1], 4),
    ];
    for (gas, cost, expected) in cases {
        assert_eq!(can_complete_circuit(gas, cost), expected);
    }
}
