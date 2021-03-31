#[derive(Default)]
pub struct WordDictionary {
    root: TrieNode,
}

impl WordDictionary {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_word(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.as_bytes() {
            let index = (c - b'a') as usize;
            node = node.table[index].get_or_insert_with(Default::default);
        }
        node.is_end = true;
    }

    pub fn search(&self, pattern: String) -> bool {
        WordDictionary::match_char(&self.root, pattern.as_bytes())
    }

    fn match_char(node: &TrieNode, pattern: &[u8]) -> bool {
        match pattern.first() {
            None => node.is_end,
            Some(&DOT) => node.table.iter().any(|child| {
                child
                    .as_ref()
                    .map(|next| WordDictionary::match_char(next, &pattern[1..]))
                    .unwrap_or(false)
            }),
            Some(&c) => node.table[(c - b'a') as usize]
                .as_ref()
                .map(|next| WordDictionary::match_char(next, &pattern[1..]))
                .unwrap_or(false),
        }
    }
}

#[derive(Default)]
struct TrieNode {
    table: [Option<Box<TrieNode>>; 26],
    is_end: bool,
}

const DOT: u8 = b'.';

#[test]
fn test_word_dictionary() {
    let words = vec!["bad", "dad", "mad"];
    let cases = vec![("pad", false), ("bad", true), (".ad", true), ("b..", true)];

    let mut wd = WordDictionary::new();
    for word in words {
        wd.add_word(word.to_string());
    }
    for (pattern, expected) in cases {
        assert_eq!(wd.search(pattern.to_string()), expected);
    }
}
