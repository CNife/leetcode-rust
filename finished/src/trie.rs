#[derive(Default)]
pub struct Trie {
    root: Node,
}

#[derive(Default)]
struct Node {
    children: [Option<Box<Node>>; 26],
    is_word: bool,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.as_bytes() {
            let idx = (c - b'a') as usize;
            let next = &mut node.children[idx];
            node = next.get_or_insert_with(Box::<Node>::default);
        }
        node.is_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        self.get_node(&word).map_or(false, |w| w.is_word)
    }
    pub fn starts_with(&self, prefix: String) -> bool {
        self.get_node(&prefix).is_some()
    }

    fn get_node(&self, s: &str) -> Option<&Node> {
        let mut node = &self.root;
        for c in s.as_bytes() {
            let idx = (c - b'a') as usize;
            match &node.children[idx] {
                Some(next) => node = next.as_ref(),
                None => return None,
            }
        }
        Some(node)
    }
}

#[test]
fn test_trie() {
    let mut t = Trie::new();
    t.insert("apple".to_string());
    assert_eq!(t.search("apple".to_string()), true);
    assert_eq!(t.search("app".to_string()), false);
    assert_eq!(t.starts_with("app".to_string()), true);
    t.insert("app".to_string());
    assert_eq!(t.search("app".to_string()), true);
}
