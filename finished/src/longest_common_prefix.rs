use std::cmp::min;

pub fn longest_common_prefix(strings: Vec<String>) -> String {
    if strings.is_empty() {
        return "".to_string();
    }

    let mut itr = strings.into_iter();
    let mut result = itr.next().unwrap();
    for string in itr {
        let prefix_len = result
            .bytes()
            .zip(string.bytes())
            .enumerate()
            .find(|(_, (lhs, rhs))| lhs != rhs)
            .map_or(min(result.len(), string.len()), |(i, _)| i);
        result.truncate(prefix_len);
        if prefix_len == 0 {
            break;
        }
    }
    result
}

#[test]
fn test() {
    use utils::vec_of;

    let cases = vec![
        (vec!["flower", "flow", "flight"], "fl"),
        (vec!["dog", "racecar", "car"], ""),
    ];
    for (strings, expect) in cases {
        assert_eq!(longest_common_prefix(vec_of(strings)), expect);
    }
}
