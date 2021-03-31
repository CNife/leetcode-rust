use std::collections::VecDeque;

pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
    let (m, n) = (image.len(), image[0].len());
    let (sr, sc) = (sr as usize, sc as usize);

    let origin_color = image[sr][sc];
    if origin_color == new_color {
        return image;
    }

    let mut queue = VecDeque::new();
    queue.push_back((sr, sc));
    while !queue.is_empty() {
        let (r, c) = queue.pop_front().unwrap();
        image[r][c] = new_color;

        macro_rules! next {
            ($predicate: expr, $r: expr, $c: expr) => {
                if $predicate && image[$r][$c] == origin_color {
                    queue.push_back(($r, $c));
                }
            };
        }
        next!(r > 0, r - 1, c);
        next!(r < m - 1, r + 1, c);
        next!(c > 0, r, c - 1);
        next!(c < n - 1, r, c + 1);
    }
    image
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]],
            1,
            1,
            2,
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]],
        ),
        (
            vec![vec![0, 0, 0], vec![0, 1, 1]],
            1,
            1,
            1,
            vec![vec![0, 0, 0], vec![0, 1, 1]],
        ),
    ];
    for (image, sr, sc, new_color, expect) in cases {
        assert_eq!(flood_fill(image, sr, sc, new_color), expect);
    }
}
