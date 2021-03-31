pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    let row_cnt: i32 = grid
        .iter()
        .map(|row| count_perimeter(row.iter().copied()))
        .sum();
    let column_cnt: i32 = (0..grid[0].len())
        .map(|c| count_perimeter(grid.iter().map(|row| row[c])))
        .sum();
    row_cnt + column_cnt
}

fn count_perimeter(itr: impl Iterator<Item = i32>) -> i32 {
    let mut flag = 0;
    let mut cnt = 0;
    for curr in itr {
        if flag != curr {
            flag = curr;
            cnt += 1;
        }
    }
    if flag == 1 {
        cnt + 1
    } else {
        cnt
    }
}

#[test]
fn test() {
    let input = vec![
        vec![0, 1, 0, 0],
        vec![1, 1, 1, 0],
        vec![0, 1, 0, 0],
        vec![1, 1, 0, 0],
    ];
    assert_eq!(island_perimeter(input), 16);
}
