use std::cmp::min;

pub fn read_binary_watch(num: i32) -> Vec<String> {
    let hour_map = vec![vec![0], vec![1, 2, 4, 8], vec![3, 5, 6, 9, 10], vec![7, 11]];
    let minute_map = vec![
        vec![0],
        vec![1, 2, 4, 8, 16, 32],
        vec![3, 5, 6, 9, 10, 12, 17, 18, 20, 24, 33, 34, 36, 40, 48],
        vec![
            7, 11, 13, 14, 19, 21, 22, 25, 26, 28, 35, 37, 38, 41, 42, 44, 49, 50, 52, 56,
        ],
        vec![15, 23, 27, 29, 30, 39, 43, 45, 46, 51, 53, 54, 57, 58],
        vec![31, 47, 55, 59],
    ];
    let num = num as usize;

    let mut result = vec![];
    for hour_ones in 0..=min(3, num) {
        let minute_ones = num - hour_ones;
        if minute_ones < 6 {
            for hour in &hour_map[hour_ones] {
                for minute in &minute_map[minute_ones] {
                    result.push(format!("{}:{:02}", hour, minute));
                }
            }
        }
    }
    result
}

#[test]
fn test() {
    use utils::assert_same_set;

    let tests = vec![(
        1,
        vec![
            "1:00", "2:00", "4:00", "8:00", "0:01", "0:02", "0:04", "0:08", "0:16", "0:32",
        ],
    )];
    for (num, want) in tests {
        let output = read_binary_watch(num);
        assert_same_set(output, want);
    }
}
