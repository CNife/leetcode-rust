pub fn find_radius(mut houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
    houses.sort_unstable();
    heaters.sort_unstable();
    heaters.push(std::i32::MAX);
    let mut max_dist = 0;
    let mut index = 0;
    for house in houses {
        while house >= heaters[index] {
            index += 1;
        }
        let curr_dist = if index > 0 {
            std::cmp::min(heaters[index] - house, house - heaters[index - 1])
        } else {
            (house - heaters[index]).abs()
        };
        max_dist = std::cmp::max(max_dist, curr_dist);
    }
    max_dist
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 3], vec![2], 1),
        (vec![1, 2, 3, 4], vec![1, 4], 1),
    ];
    for (houses, heaters, expected) in cases {
        assert_eq!(find_radius(houses, heaters), expected);
    }
}
