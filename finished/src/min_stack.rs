use std::cmp::min;

pub struct MinStack(Vec<MinStackNode>);

struct MinStackNode {
    value: i32,
    min: i32,
}

impl MinStack {
    pub fn new() -> MinStack {
        MinStack(Vec::new())
    }

    pub fn push(&mut self, value: i32) {
        let min = self.0.last().map_or(value, |node| min(node.min, value));
        self.0.push(MinStackNode { value, min });
    }

    pub fn pop(&mut self) {
        let _ = self.0.pop();
    }

    pub fn top(&self) -> i32 {
        self.0.last().map_or(-1, |node| node.value)
    }

    pub fn get_min(&self) -> i32 {
        self.0.last().map_or(-1, |node| node.min)
    }
}

#[test]
fn test() {
    let mut ms = MinStack::new();
    ms.push(-2);
    ms.push(0);
    ms.push(-3);
    assert_eq!(ms.get_min(), -3);
    ms.pop();
    assert_eq!(ms.top(), 0);
    assert_eq!(ms.get_min(), -2);
}

#[test]
fn test_min_stack_node() {
    use std::mem;

    assert_eq!(mem::size_of::<MinStackNode>(), 8);
    assert_eq!(mem::align_of::<MinStackNode>(), 4);
}
