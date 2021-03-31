use std::cmp::Ordering::*;

pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
    let mut min_diff = std::i32::MAX;
    let mut idxs = vec![];

    arr.sort_unstable();
    for i in 0..arr.len() - 1 {
        let diff = arr[i + 1] - arr[i];
        match diff.cmp(&min_diff) {
            Equal => idxs.push(i),
            Less => {
                min_diff = diff;
                idxs.clear();
                idxs.push(i);
            }
            Greater => {}
        }
    }

    idxs.into_iter().map(|i| vec![arr[i], arr[i + 1]]).collect()
}

#[test]
fn test() {
    let cases = vec![
        (vec![4, 2, 1, 3], vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
        (vec![1, 3, 6, 10, 15], vec![vec![1, 3]]),
        (
            vec![3, 8, -10, 23, 19, -4, -14, 27],
            vec![vec![-14, -10], vec![19, 23], vec![23, 27]],
        ),
    ];
    for (input, expected) in cases {
        assert_eq!(minimum_abs_difference(input), expected);
    }
}
