use std::collections::HashMap;

pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
    let mut dist_map = HashMap::new();
    let mut res = 0;
    for i in 0..points.len() {
        dist_map.clear();
        for j in 0..points.len() {
            if i == j {
                continue;
            }
            let dist = distance(&points[i], &points[j]);
            let cnt = dist_map.entry(dist).or_insert(0);
            res += *cnt * 2;
            *cnt += 1;
        }
    }
    res
}

fn distance(p1: &[i32], p2: &[i32]) -> i32 {
    let x_distance = p1[0] - p2[0];
    let y_distance = p1[1] - p2[1];
    x_distance * x_distance + y_distance * y_distance
}

#[test]
fn test() {
    let cases = vec![(vec![vec![0, 0], vec![1, 0], vec![2, 0]], 2)];
    for (points, expected) in cases {
        assert_eq!(number_of_boomerangs(points), expected);
    }
}
