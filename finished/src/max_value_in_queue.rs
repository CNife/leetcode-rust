use std::collections::VecDeque;

#[derive(Default)]
pub struct MaxQueue {
    values: VecDeque<i32>,
    maxes: VecDeque<i32>,
}

impl MaxQueue {
    pub fn new() -> MaxQueue {
        MaxQueue::default()
    }

    pub fn max_value(&self) -> i32 {
        self.maxes.front().map_or(-1, |i| *i)
    }

    pub fn push_back(&mut self, value: i32) {
        self.values.push_back(value);

        while let Some(back) = self.maxes.back() {
            if *back < value {
                self.maxes.pop_back();
            } else {
                break;
            }
        }
        self.maxes.push_back(value);
    }

    pub fn pop_front(&mut self) -> i32 {
        match (self.values.pop_front(), self.maxes.front()) {
            (Some(result), Some(front)) => {
                if result == *front {
                    self.maxes.pop_front();
                }
                result
            }
            (None, None) => -1,
            _ => unreachable!(),
        }
    }
}

#[test]
fn test1() {
    let mut q = MaxQueue::new();
    q.push_back(1);
    q.push_back(2);
    assert_eq!(q.max_value(), 2);
    assert_eq!(q.pop_front(), 1);
    assert_eq!(q.max_value(), 2);
}

#[test]
fn test2() {
    let mut q = MaxQueue::new();
    assert_eq!(q.pop_front(), -1);
    assert_eq!(q.max_value(), -1);
}

#[test]
fn test3() {
    let mut q = MaxQueue::new();
    q.push_back(15);
    assert_eq!(q.max_value(), 15);
    q.push_back(9);
    assert_eq!(q.max_value(), 15);
}
