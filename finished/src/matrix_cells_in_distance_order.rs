pub fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
    let mut vp = Vec::new();
    for i in 0..r {
        for j in 0..c {
            let dist = ((i - r0).abs() + (j - c0).abs()) as usize;
            if dist >= vp.len() {
                for _ in vp.len()..=dist {
                    vp.push(Vec::new());
                }
            }
            vp[dist].push(vec![i, j]);
        }
    }
    vp.into_iter()
        .flat_map(|points| points.into_iter())
        .collect()
}

#[test]
fn test() {
    use utils::assert_same_set;

    let tests = vec![
        (1, 2, 0, 0, vec![vec![(0, 0)], vec![(0, 1)]]),
        (
            2,
            2,
            0,
            1,
            vec![vec![(0, 1)], vec![(0, 0), (1, 1)], vec![(1, 0)]],
        ),
        (
            2,
            3,
            1,
            2,
            vec![
                vec![(1, 2)],
                vec![(0, 2), (1, 1)],
                vec![(0, 1), (1, 0)],
                vec![(0, 0)],
            ],
        ),
    ];
    for (r, c, r0, c0, want) in tests {
        let output = all_cells_dist_order(r, c, r0, c0);
        let mut i = 0usize;
        for ring in want {
            let ring_want: Vec<_> = ring.into_iter().map(|(x, y)| vec![x, y]).collect();
            let ring_output = output[i..i + ring_want.len()].to_vec();
            i += ring_want.len();
            assert_same_set(ring_want, ring_output);
        }
    }
}
