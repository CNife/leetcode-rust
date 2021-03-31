use std::collections::{HashMap, HashSet, VecDeque};

pub fn calc_equation(
    equations: Vec<Vec<String>>,
    values: Vec<f64>,
    queries: Vec<Vec<String>>,
) -> Vec<f64> {
    let mut graph = Graph::default();
    for (equation, value) in equations.iter().zip(values.into_iter()) {
        graph.add(&equation[0], &equation[1], value);
    }
    queries
        .into_iter()
        .map(|query| graph.calc(&query[0], &query[1]))
        .collect()
}

#[derive(Default)]
struct Graph<'a> {
    edges: HashMap<&'a str, Vec<(&'a str, f64)>>,
}

impl<'a> Graph<'a> {
    fn add(&mut self, from: &'a str, to: &'a str, value: f64) {
        let mut add_edge = |from: &'a str, to: &'a str, value: f64| {
            self.edges
                .entry(from)
                .or_insert_with(Vec::new)
                .push((to, value));
        };
        add_edge(from, to, value);
        add_edge(to, from, value.recip());
    }

    fn calc(&self, from: &'a str, to: &'a str) -> f64 {
        if !self.edges.contains_key(&from) || !self.edges.contains_key(&to) {
            return -1.0;
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((from, 1.0));
        while let Some((node, curr)) = queue.pop_front() {
            for &(next, value) in self.edges.get(&node).unwrap() {
                if next == to {
                    return curr * value;
                }
                if !visited.contains(&next) {
                    visited.insert(next);
                    queue.push_back((next, curr * value));
                }
            }
        }
        -1.0
    }
}

#[test]
fn test() {
    use utils::vec_of_vec_of;

    let cases = vec![
        (
            vec![vec!["a", "b"], vec!["b", "c"]],
            vec![2.0, 3.0],
            vec![
                vec!["a", "c"],
                vec!["b", "a"],
                vec!["a", "e"],
                vec!["a", "a"],
                vec!["x", "x"],
            ],
            vec![6.0, 0.5, -1.0, 1.0, -1.0],
        ),
        (
            vec![vec!["a", "b"], vec!["c", "d"]],
            vec![1.0, 1.0],
            vec![
                vec!["a", "c"],
                vec!["b", "d"],
                vec!["b", "a"],
                vec!["d", "c"],
            ],
            vec![-1.0, -1.0, 1.0, 1.0],
        ),
    ];
    for (equations, values, queries, expected) in cases {
        assert_eq!(
            calc_equation(vec_of_vec_of(equations), values, vec_of_vec_of(queries)),
            expected
        );
    }
}
