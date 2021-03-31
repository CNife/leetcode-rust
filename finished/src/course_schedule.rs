pub fn can_finish(node_count: i32, edges: Vec<Vec<i32>>) -> bool {
    let num_courses = node_count as usize;
    let mut graph = vec![vec![]; num_courses];
    let mut in_degrees = vec![0; num_courses];
    for edge in edges {
        let from = edge[0] as usize;
        let to = edge[1] as usize;
        graph[from].push(to);
        in_degrees[to] += 1;
    }

    while let Some((start_node, _)) = in_degrees
        .iter()
        .enumerate()
        .find(|(_, in_degree)| **in_degree == 0)
    {
        in_degrees[start_node] = -1;
        for &end_node in &graph[start_node] {
            in_degrees[end_node] -= 1;
        }
    }

    in_degrees.into_iter().all(|n| n <= 0)
}

#[test]
fn test() {
    let cases = vec![
        (2, vec![vec![0, 1]], true),
        (2, vec![vec![1, 0]], true),
        (2, vec![vec![1, 0], vec![0, 1]], false),
    ];
    for (num_courses, prerequisites, expected) in cases {
        assert_eq!(can_finish(num_courses, prerequisites), expected);
    }
}
