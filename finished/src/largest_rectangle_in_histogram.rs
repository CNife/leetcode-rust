//pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
//    (0..heights.len())
//        .map(|i| fuck(&heights, i))
//        .max()
//        .unwrap_or(0)
//}
//
//fn fuck(heights: &[i32], i: usize) -> i32 {
//    let height = heights[i];
//    let left = (0..i)
//        .rev()
//        .find(|j| heights[*j] < height)
//        .map_or(0, |j| j + 1);
//    let right = (i..heights.len())
//        .find(|j| heights[*j] < height)
//        .unwrap_or(heights.len());
//    height * (right - left) as i32
//}

use std::cmp::max;

pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    calculate_largest_area(&heights)
}

fn calculate_largest_area(heights: &[i32]) -> i32 {
    match heights.len() {
        0 => 0,
        1 => heights[0],
        len => {
            let (min_index, &min_height) = heights
                .iter()
                .enumerate()
                .min_by_key(|(_, &height)| height)
                .unwrap();

            let center_area = min_height * len as i32;
            let left_area = calculate_largest_area(&heights[..min_index]);
            let right_area = calculate_largest_area(&heights[min_index + 1..]);
            max(center_area, max(left_area, right_area))
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![2, 1, 5, 6, 2, 3], 10),
        (vec![6, 7, 5, 2, 4, 5, 9, 3], 16),
    ];
    for (heights, expected) in cases {
        assert_eq!(largest_rectangle_area(heights), expected);
    }
}
