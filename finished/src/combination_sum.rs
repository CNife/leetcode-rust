pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    candidates.sort_unstable();
    let mut results = vec![];
    let mut stack = vec![];
    backtrace(&candidates, target, &mut stack, &mut results);
    results
}

fn backtrace(candidates: &[i32], target: i32, stack: &mut Vec<i32>, results: &mut Vec<Vec<i32>>) {
    for (index, &candidate) in candidates.iter().enumerate() {
        if candidate <= target {
            stack.push(candidate);
            if candidate < target {
                backtrace(&candidates[index..], target - candidate, stack, results);
            } else {
                results.push(stack.clone());
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
        (vec![2, 3, 6, 7], 7, vec![vec![7], vec![2, 2, 3]]),
        (
            vec![2, 3, 5],
            8,
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]],
        ),
    ];
    for (candidates, target, want) in tests {
        let output = combination_sum(candidates, target);
        assert_same_set(output, want);
    }
}
