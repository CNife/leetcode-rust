pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
    let n = points.len();
    let mut max_area = 0f64;
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            for k in j + 1..n {
                let area = area(
                    points[i][0],
                    points[i][1],
                    points[j][0],
                    points[j][1],
                    points[k][0],
                    points[k][1],
                );
                if max_area < area {
                    max_area = area;
                }
            }
        }
    }
    max_area
}

fn area(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) -> f64 {
    let dx1 = x2 - x1;
    let dx2 = x3 - x1;
    let dy1 = y2 - y1;
    let dy2 = y3 - y1;
    (dx1 * dy2 - dx2 * dy1).abs() as f64 / 2.0
}

#[test]
fn test() {
    let points = vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![0, 2], vec![2, 0]];
    assert!((largest_triangle_area(points) - 2.0).abs() < 1e-6);
}
