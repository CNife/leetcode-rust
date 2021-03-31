// 分治解法
//pub fn partition(input: String) -> Vec<Vec<String>> {
//    if input.is_empty() {
//        vec![]
//    } else {
//        do_partition(&input, 0)
//    }
//}
//
//fn do_partition(input: &str, split: usize) -> Vec<Vec<String>> {
//    if split == input.len() {
//        return vec![vec![]];
//    }
//    let mut result = vec![];
//    for i in split..input.len() {
//        let prev = &input[split..=i];
//        if is_palindrome(prev) {
//            for mut s in do_partition(input, split + 1) {
//                s.insert(0, prev.to_string());
//                result.push(s);
//            }
//        }
//    }
//    result
//}
//
//fn is_palindrome(input: &str) -> bool {
//    input
//        .bytes()
//        .zip(input.bytes().rev())
//        .take(input.len() / 2)
//        .all(|(lhs, rhs)| lhs == rhs)
//}

pub fn partition(input: String) -> Vec<Vec<String>> {
    let mut results = vec![];
    let mut stack = vec![];
    backtrace(input.as_bytes(), 0, &mut stack, &mut results);
    results
}

fn backtrace<'a>(
    input: &'a [u8],
    start: usize,
    stack: &mut Vec<&'a [u8]>,
    results: &mut Vec<Vec<String>>,
) {
    if start < input.len() {
        for j in start + 1..=input.len() {
            let slice = &input[start..j];
            if is_palindrome(slice) {
                stack.push(slice);
                backtrace(input, j, stack, results);
                stack.pop().unwrap();
            }
        }
    } else {
        let result: Vec<String> = stack
            .iter()
            .map(|&slice| unsafe { String::from_utf8_unchecked(slice.into()) })
            .collect();
        results.push(result);
    }
}

//noinspection ALL
fn is_palindrome(input: &[u8]) -> bool {
    input
        .iter()
        .zip(input.iter().rev())
        .take(input.len() / 2)
        .all(|(lhs, rhs)| lhs == rhs)
}

#[cfg(test)]
mod test {
    use std::collections::BTreeSet;

    #[test]
    fn test() {
        let cases = vec![("aab", vec![vec!["aa", "b"], vec!["a", "a", "b"]])];
        for (input, expected) in cases {
            assert_eq!(set(super::partition(input.to_string())), set(expected));
        }
    }

    fn set<S: ToString>(vvs: Vec<Vec<S>>) -> BTreeSet<Vec<String>> {
        let mut set = BTreeSet::new();
        for vs in vvs {
            let mut vs = vs.into_iter().map(|s| s.to_string()).collect::<Vec<_>>();
            vs.sort_unstable();
            set.insert(vs);
        }
        set
    }
}
