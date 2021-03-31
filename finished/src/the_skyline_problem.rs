pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut points = Vec::with_capacity(buildings.len() * 2);
    for building in buildings {
        points.push((building[0], -building[2]));
        points.push((building[1], building[2]));
    }
    points.sort_unstable();

    let mut result = Vec::new();
    let mut heights = vec![0];
    let mut last_height = 0;

    for (index, height) in points {
        if height < 0 {
            let real_height = -height;
            let insert_index = heights.binary_search(&real_height).unwrap_or_else(|i| i);
            heights.insert(insert_index, real_height);
        } else {
            let remove_index = heights.binary_search(&height).unwrap();
            heights.remove(remove_index);
        }

        let max_height = *heights.last().unwrap();
        if max_height != last_height {
            result.push(vec![index, max_height]);
            last_height = max_height;
        }
    }

    result
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![
                vec![2, 9, 10],
                vec![3, 7, 15],
                vec![5, 12, 12],
                vec![15, 20, 10],
                vec![19, 24, 8],
            ],
            vec![
                vec![2, 10],
                vec![3, 15],
                vec![7, 12],
                vec![12, 0],
                vec![15, 10],
                vec![20, 8],
                vec![24, 0],
            ],
        ),
        (
            vec![vec![0, 2, 3], vec![2, 5, 3]],
            vec![vec![0, 3], vec![5, 0]],
        ),
    ];
    for (buildings, expected) in cases {
        assert_eq!(get_skyline(buildings), expected);
    }
}
