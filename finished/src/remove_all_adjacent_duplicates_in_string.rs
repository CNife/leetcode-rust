pub fn remove_duplicates<S>(s: S) -> String
where
    S: AsRef<[u8]>,
{
    let s = s.as_ref();
    let mut stack = vec![];
    for ch in s {
        match stack.last() {
            Some(top) if top == ch => {
                stack.pop();
            }
            _ => stack.push(*ch),
        };
    }
    unsafe { String::from_utf8_unchecked(stack) }
}

#[test]
fn test() {
    assert_eq!(remove_duplicates("abbaca"), "ca");
}
