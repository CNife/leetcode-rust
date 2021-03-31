pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    let mut index = match letters.binary_search(&target) {
        Ok(i) => i + 1,
        Err(i) => i,
    };
    if index >= letters.len() {
        index = 0;
    }
    letters[index]
}

#[test]
fn test() {
    let cases = vec![
        (vec!['c', 'f', 'j'], 'a', 'c'),
        (vec!['c', 'f', 'j'], 'c', 'f'),
        (vec!['c', 'f', 'j'], 'd', 'f'),
        (vec!['c', 'f', 'j'], 'g', 'j'),
        (vec!['c', 'f', 'j'], 'j', 'c'),
        (vec!['c', 'f', 'j'], 'k', 'c'),
    ];
    for (letters, target, expect) in cases {
        assert_eq!(next_greatest_letter(letters, target), expect);
    }
}
