use std::collections::HashSet;

pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    candidates.sort_unstable();
    let mut results = HashSet::new();
    let mut stack = vec![];
    backtrace(&candidates, target, &mut stack, &mut results);
    results.into_iter().collect()
}

fn backtrace(
    candidates: &[i32],
    target: i32,
    stack: &mut Vec<i32>,
    results: &mut HashSet<Vec<i32>>,
) {
    for (index, &candidate) in candidates.iter().enumerate() {
        if candidate <= target {
            stack.push(candidate);
            if candidate < target {
                backtrace(&candidates[index + 1..], target - candidate, stack, results);
            } else {
                results.insert(stack.clone());
            }
            stack.pop().unwrap();
        } else {
            break;
        }
    }
}

#[test]
fn test() {
    use utils::assert_same_set;

    let tests = vec![
        (
            vec![10, 1, 2, 7, 6, 1, 5],
            8,
            vec![vec![1, 7], vec![1, 2, 5], vec![2, 6], vec![1, 1, 6]],
        ),
        (vec![2, 5, 2, 1, 2], 5, vec![vec![1, 2, 2], vec![5]]),
    ];
    for (candidates, target, want) in tests {
        let output = combination_sum2(candidates, target);
        assert_same_set(output, want);
    }
}
