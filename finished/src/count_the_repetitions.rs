use std::collections::HashMap;

pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
    let s1 = s1.into_bytes();
    let s2 = s2.into_bytes();
    let len1 = s1.len();
    let len2 = s2.len();
    let n1 = n1 as usize;
    let n2 = n2 as usize;
    if len1 == 0 || len2 == 0 || len1 * n1 < len2 * n2 {
        return 0;
    }

    let mut map1: HashMap<usize, usize> = HashMap::new();
    let mut map2: HashMap<usize, usize> = HashMap::new();
    let mut index1 = 0;
    let mut index2 = 0;
    let mut result = 0;
    while index1 / len1 < n1 {
        if index1 % len1 == len1 - 1 {
            match map1.get(&(index2 % len2)) {
                Some(val) => {
                    let cycle_len = index1 / len1 - val / len1;
                    let cycle_num = (n1 - 1 - index1 / len1) / cycle_len;
                    let cycle_s2_num = index2 / len2 - map2.get(&(index2 % len2)).unwrap() / len2;
                    index1 += cycle_num * cycle_len * len1;
                    result += cycle_num * cycle_s2_num;
                }
                None => {
                    map1.insert(index2 % len2, index1);
                    map2.insert(index2 % len2, index2);
                }
            }
        }
        if s1[index1 % len1] == s2[index2 % len2] {
            if index2 % len2 == len2 - 1 {
                result += 1;
            }
            index2 += 1;
        }
        index1 += 1;
    }
    (result / n2) as i32
}

#[test]
fn test() {
    let cases = vec![("acb", 4, "ab", 2, 2)];
    for (s1, n1, s2, n2, expect) in cases {
        assert_eq!(get_max_repetitions(s1.into(), n1, s2.into(), n2), expect);
    }
}
