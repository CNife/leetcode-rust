use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};

#[derive(Debug)]
pub struct LFUCache {
    map: HashMap<i32, Node>,
    set: BTreeSet<Node>,
    capacity: usize,
    time: i32,
}

impl LFUCache {
    pub fn new(capacity: i32) -> LFUCache {
        LFUCache {
            map: HashMap::new(),
            set: BTreeSet::new(),
            capacity: capacity as usize,
            time: 0,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        self.time += 1;
        match self.map.get_mut(&key) {
            Some(node) => {
                self.set.remove(node);
                node.frequency += 1;
                node.last_accessed = self.time;
                self.set.insert(node.clone());
                node.value
            }
            None => -1,
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        self.time += 1;
        match self.map.get_mut(&key) {
            Some(node) => {
                self.set.remove(node);
                node.frequency += 1;
                node.last_accessed = self.time;
                node.value = value;
                self.set.insert(node.clone());
            }
            None => {
                if self.set.len() < self.capacity {
                    let node = Node::new(key, value, self.time);
                    self.map.insert(key, node.clone());
                    self.set.insert(node);
                } else if let Some(first) = self.set.iter().next() {
                    let fist_copy = first.clone();
                    self.set.remove(&fist_copy);
                    self.map.remove(&fist_copy.key);
                    let node = Node::new(key, value, self.time);
                    self.map.insert(key, node.clone());
                    self.set.insert(node);
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Node {
    key: i32,
    value: i32,
    frequency: i32,
    last_accessed: i32,
}

impl Node {
    fn new(key: i32, value: i32, time: i32) -> Node {
        Node {
            key,
            value,
            frequency: 1,
            last_accessed: time,
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.frequency.partial_cmp(&other.frequency) {
            Some(Ordering::Equal) => self.last_accessed.partial_cmp(&other.last_accessed),
            prev => prev,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency
            .cmp(&other.frequency)
            .then(self.last_accessed.cmp(&other.last_accessed))
    }
}

#[test]
fn test() {
    let mut cache = LFUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    assert_eq!(cache.get(1), 1);
    cache.put(3, 3);
    assert_eq!(cache.get(2), -1);
    assert_eq!(cache.get(3), 3);
    cache.put(4, 4);
    assert_eq!(cache.get(1), -1);
    assert_eq!(cache.get(3), 3);
    assert_eq!(cache.get(4), 4);
}
