//use std::collections::VecDeque;
//use std::iter::FromIterator;
//
//pub fn find_order(n: i32, raw_edges: Vec<Vec<i32>>) -> Vec<i32> {
//    let n = n as usize;
//    let mut result = Vec::with_capacity(n);
//    let mut in_degrees = vec![0; n];
//    let mut edges = vec![vec![]; n];
//    for edge in &raw_edges {
//        edges[edge[0] as usize].push(edge[1] as usize);
//        in_degrees[edge[1] as usize] += 1;
//    }
//
//    let mut queue = VecDeque::from_iter(
//        in_degrees
//            .iter()
//            .enumerate()
//            .filter(|(_, in_degree)| **in_degree == 0)
//            .map(|(node, _)| node),
//    );
//
//    while let Some(node) = queue.pop_front() {
//        result.push(node as i32);
//        for &to_node in &edges[node] {
//            in_degrees[to_node] -= 1;
//            if in_degrees[to_node] == 0 {
//                queue.push_back(to_node);
//            }
//        }
//    }
//
//    if result.len() == n {
//        result.reverse();
//    } else {
//        result.clear();
//    }
//    result
//}

pub fn find_order(n: i32, raw_edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut graph = vec![vec![]; n];
    for edge in raw_edges {
        graph[edge[0] as usize].push(edge[1] as usize);
    }
    let mut visit_flags = vec![VisitFlag::NotVisited; n];
    let mut result = Vec::with_capacity(n);
    for node in 0..n {
        if let VisitFlag::NotVisited = visit_flags[node] {
            if let Err(LoopError) = dfs(node, &graph, &mut visit_flags, &mut result) {
                return vec![];
            }
        }
    }
    result
}

#[derive(Copy, Clone)]
enum VisitFlag {
    NotVisited,
    Visiting,
    Visited,
}

struct LoopError;

fn dfs(
    start: usize,
    graph: &[Vec<usize>],
    visit_flags: &mut Vec<VisitFlag>,
    result: &mut Vec<i32>,
) -> Result<(), LoopError> {
    match visit_flags[start] {
        VisitFlag::NotVisited => {
            visit_flags[start] = VisitFlag::Visiting;
            for end in &graph[start] {
                dfs(*end, graph, visit_flags, result)?;
            }
            visit_flags[start] = VisitFlag::Visited;
            result.push(start as i32);
            Ok(())
        }
        VisitFlag::Visited => Ok(()),
        VisitFlag::Visiting => Err(LoopError),
    }
}

#[test]
fn test() {
    let cases = vec![
        (2, vec![vec![1, 0]], vec![0, 1]),
        (
            4,
            vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]],
            vec![0, 1, 2, 3],
        ),
    ];
    for (num_courses, prerequisites, expected) in cases {
        assert_eq!(find_order(num_courses, prerequisites), expected);
    }
}
