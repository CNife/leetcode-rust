use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct DinnerPlates {
    cap: usize,
    stacks: Vec<Vec<i32>>,
    half_fulls: BinaryHeap<Reverse<usize>>,
}

impl DinnerPlates {
    pub fn new(cap: i32) -> Self {
        DinnerPlates {
            cap: cap as usize,
            stacks: Default::default(),
            half_fulls: Default::default(),
        }
    }

    pub fn push(&mut self, val: i32) {
        match self.half_fulls.peek() {
            Some(Reverse(i)) => {
                self.stacks[*i].push(val);
                if self.stacks[*i].len() >= self.cap {
                    self.half_fulls.pop();
                }
            }
            None => {
                if self.stacks.is_empty() || self.stacks.last().unwrap().len() >= self.cap {
                    self.stacks.push(vec![]);
                }
                self.stacks.last_mut().unwrap().push(val);
            }
        }
    }

    pub fn pop(&mut self) -> i32 {
        let result = self
            .stacks
            .iter_mut()
            .rfind(|stack| !stack.is_empty())
            .and_then(|stack| stack.pop())
            .unwrap_or(-1);
        while self.stacks.len() > 1 {
            match self.stacks.last() {
                Some(stack) if stack.is_empty() => {
                    self.stacks.pop();
                }
                _ => break,
            }
        }
        result
    }

    pub fn pop_at_stack(&mut self, index: i32) -> i32 {
        let index = index as usize;
        if index >= self.stacks.len() {
            return -1;
        }
        if index < self.stacks.len() - 1 && self.stacks[index].len() >= self.cap {
            self.half_fulls.push(Reverse(index));
        }
        self.stacks[index].pop().unwrap_or(-1)
    }
}

#[test]
fn test() {
    let mut d = DinnerPlates::new(2);
    d.push(1);
    d.push(2);
    d.push(3);
    d.push(4);
    d.push(5);
    assert_eq!(d.pop_at_stack(0), 2);
    d.push(20);
    d.push(21);
    assert_eq!(d.pop_at_stack(0), 20);
    assert_eq!(d.pop_at_stack(2), 21);
    d.pop();
    d.pop();
    d.pop();
    d.pop();
    d.pop();
}
