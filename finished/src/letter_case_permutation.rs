pub fn letter_case_permutation(s: String) -> Vec<String> {
    let mut res = vec![];
    do_permutation(s.into_bytes(), 0, &mut res);
    res
}

fn do_permutation(s: Vec<u8>, idx: usize, res: &mut Vec<String>) {
    use std::cmp::Ordering::*;

    match idx.cmp(&s.len()) {
        Equal => {
            let result = unsafe { String::from_utf8_unchecked(s) };
            res.push(result);
        }
        Less => {
            let ch = s[idx];
            if ch.is_ascii_uppercase() {
                let mut sc = s.clone();
                sc[idx] = ch.to_ascii_lowercase();
                do_permutation(sc, idx + 1, res);
            } else if ch.is_ascii_lowercase() {
                let mut sc = s.clone();
                sc[idx] = ch.to_ascii_uppercase();
                do_permutation(sc, idx + 1, res);
            }
            do_permutation(s, idx + 1, res);
        }
        Greater => {}
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    fn hash_set<S: ToString>(vs: Vec<S>) -> HashSet<String> {
        vs.into_iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test() {
        let cases = vec![
            ("a1b2", vec!["a1b2", "a1B2", "A1b2", "A1B2"]),
            ("3z4", vec!["3z4", "3Z4"]),
            ("12345", vec!["12345"]),
        ];
        for (s, expected) in cases {
            let output = hash_set(letter_case_permutation(s.to_string()));
            assert_eq!(output, hash_set(expected));
        }
    }
}
