pub struct RLEIterator {
    iter: VecIter,
    item: i32,
    count: i32,
}

type VecIter = <Vec<i32> as IntoIterator>::IntoIter;

impl RLEIterator {
    pub fn new(code: Vec<i32>) -> Self {
        let mut iter = code.into_iter();
        let (count, item) = RLEIterator::next_pair(&mut iter);
        RLEIterator { iter, item, count }
    }

    pub fn next(&mut self, n: i32) -> i32 {
        match self.count {
            -1 => -1,
            last_count if last_count < n => {
                let (next_count, next_item) = RLEIterator::next_pair(&mut self.iter);
                self.count = next_count;
                self.item = next_item;
                self.next(n - last_count)
            }
            _ => {
                self.count -= n;
                self.item
            }
        }
    }

    fn next_pair(iter: &mut VecIter) -> (i32, i32) {
        let count = iter.next().unwrap_or(-1);
        let item = iter.next().unwrap_or(-1);
        (count, item)
    }
}

#[test]
fn test() {
    let mut itr = RLEIterator::new(vec![3, 8, 0, 9, 2, 5]);
    assert_eq!(itr.next(2), 8);
    assert_eq!(itr.next(1), 8);
    assert_eq!(itr.next(1), 5);
    assert_eq!(itr.next(2), -1);
}
