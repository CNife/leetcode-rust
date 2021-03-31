pub struct Solution;

impl Solution {
    pub fn str_without3a3b(mut a: i32, mut b: i32) -> String {
        let mut res = Vec::with_capacity((a + b) as usize);
        let mut i = 0;
        while a > 0 || b > 0 {
            let write_a = if i >= 2 && res[i - 1] == res[i - 2] {
                res[i - 1] == b'b'
            } else {
                a > b
            };
            if write_a {
                a -= 1;
                res.push(b'a');
            } else {
                b -= 1;
                res.push(b'b');
            }
            i += 1;
        }
        unsafe { String::from_utf8_unchecked(res) }
    }
}

#[test]
fn test_str_without_3a3b() {
    let inputs = vec![(1, 2), (4, 1), (6, 2)];
    for (a, b) in inputs {
        let result = Solution::str_without3a3b(a, b);
        assert_eq!(result.len(), (a + b) as usize);
        assert_eq!(result.chars().filter(|&c| c == 'a').count(), a as usize);
        assert_eq!(result.chars().filter(|&c| c == 'b').count(), b as usize);
        assert_eq!(result.find("aaa"), None);
        assert_eq!(result.find("bbb"), None);
    }
}
