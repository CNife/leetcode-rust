use std::cmp::min;
use std::collections::{HashSet, VecDeque};

pub fn can_measure_water(x: i32, y: i32, z: i32) -> bool {
    let next_state = |state: (i32, i32), op: u8| -> (i32, i32) {
        match (op, state) {
            (0, (_, y_now)) => (x, y_now),
            (1, (x_now, _)) => (x_now, y),
            (2, (_, y_now)) => (0, y_now),
            (3, (x_now, _)) => (x_now, 0),
            (4, (x_now, y_now)) => {
                let y_next = min(x_now + y_now, y);
                let x_next = x_now + y_now - y_next;
                (x_next, y_next)
            }
            (5, (x_now, y_now)) => {
                let x_next = min(x_now + y_now, x);
                let y_next = x_now + y_now - x_next;
                (x_next, y_next)
            }
            _ => unreachable!(),
        }
    };

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((0, 0));
    while let Some(state) = queue.pop_front() {
        if state.0 + state.1 == z {
            return true;
        }

        visited.insert(state);
        for op in 0..6 {
            let next = next_state(state, op);
            if !visited.contains(&next) {
                queue.push_back(next);
            }
        }
    }
    false
}

#[test]
fn test() {
    let cases = vec![(3, 5, 4, true), (2, 6, 5, false)];
    for (x, y, z, expected) in cases {
        assert_eq!(can_measure_water(x, y, z), expected);
    }
}
