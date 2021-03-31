use std::iter::FromIterator;

pub struct CombinationIterator {
    chars: Vec<u8>,
    index_stack: Vec<usize>,
    has_next: bool,
}

impl CombinationIterator {
    pub fn new(characters: String, combination_length: i32) -> Self {
        CombinationIterator {
            chars: characters.into_bytes(),
            index_stack: Vec::from_iter(0..combination_length as usize),
            has_next: true,
        }
    }

    pub fn has_next(&self) -> bool {
        self.has_next
    }

    pub fn next(&mut self) -> String {
        if self.has_next {
            let result: Vec<u8> = self
                .index_stack
                .iter()
                .map(|index| self.chars[*index])
                .collect();
            if next_indexes(&mut self.index_stack, self.chars.len() - 1).is_none() {
                self.has_next = false;
            }
            unsafe { String::from_utf8_unchecked(result) }
        } else {
            String::new()
        }
    }
}

fn next_indexes(stack: &mut Vec<usize>, max: usize) -> Option<usize> {
    match stack.pop() {
        None => None,
        Some(prev) if prev < max => {
            stack.push(prev + 1);
            Some(prev + 1)
        }
        _ => match next_indexes(stack, max - 1) {
            Some(left) if left < max => {
                stack.push(left + 1);
                Some(left + 1)
            }
            _ => None,
        },
    }
}

#[test]
fn test_next_indexes() {
    let n: usize = 2;
    let m: usize = 4;
    let mut stack = Vec::from_iter(0..n);
    loop {
        if !next_indexes(&mut stack, m - 1).is_some() {
            break;
        }
    }
}

#[test]
fn test() {
    let mut ci = CombinationIterator::new("abc".to_string(), 2);
    assert_eq!(ci.next(), "ab");
    assert!(ci.has_next());
    assert_eq!(ci.next(), "ac");
    assert!(ci.has_next());
    assert_eq!(ci.next(), "bc");
    assert!(!ci.has_next());
}
