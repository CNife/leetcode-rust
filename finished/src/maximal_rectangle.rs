use std::cmp::max;

pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }

    let mut max_area = 0;
    let mut dp = vec![0; matrix[0].len()];

    for row in matrix {
        for (dpn, ch) in dp.iter_mut().zip(row.into_iter()) {
            *dpn = if ch == '1' { *dpn + 1 } else { 0 };
        }
        max_area = max(max_area, largest_rectangle_in_histogram(&dp));
    }

    max_area
}

fn largest_rectangle_in_histogram(heights: &[i32]) -> i32 {
    let mut stack = vec![-1];
    let mut max_area = 0;

    for (i, &height) in heights.iter().enumerate() {
        loop {
            let top = *stack.last().unwrap();
            if top != -1 && heights[top as usize] >= height {
                stack.pop();
                let new_top = *stack.last().unwrap();
                let area = heights[top as usize] * (i as i32 - new_top - 1);
                max_area = max(max_area, area);
            } else {
                break;
            }
        }
        stack.push(i as i32);
    }

    loop {
        let top = stack.pop().unwrap();
        if top != -1 {
            let new_top = *stack.last().unwrap();
            let area = heights[top as usize] * (heights.len() as i32 - new_top - 1);
            max_area = max(max_area, area);
        } else {
            break;
        }
    }
    max_area
}

#[test]
fn test() {
    let cases = vec![(
        vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ],
        6,
    )];
    for (matrix, expected) in cases {
        assert_eq!(maximal_rectangle(matrix), expected);
    }
}
