pub fn min_cost_to_move_chips(chips: Vec<i32>) -> i32 {
    let even_cnt = chips.iter().filter(|&n| n & 1 == 0).count();
    let odd_cnt = chips.len() - even_cnt;
    std::cmp::min(even_cnt, odd_cnt) as i32
}

#[test]
fn test() {
    assert_eq!(min_cost_to_move_chips(vec![1, 2, 3]), 1);
    assert_eq!(min_cost_to_move_chips(vec![2, 2, 2, 3, 3]), 2);
    assert_eq!(min_cost_to_move_chips(vec![1, 2, 2, 2, 2]), 1);
}
