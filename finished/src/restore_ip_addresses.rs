// 暴力解法
//pub fn restore_ip_addresses(s: String) -> Vec<String> {
//    if s.len() < 4 {
//        return vec![];
//    }
//
//    let is_valid = |start: usize, end: usize| -> bool {
//        let segment = &s[start..end];
//        match segment.parse::<u8>() {
//            Ok(n) => {
//                if n == 0 {
//                    segment.len() == 1
//                } else {
//                    !segment.starts_with('0')
//                }
//            }
//            Err(_) => false,
//        }
//    };
//
//    let mut result = vec![];
//    for a in 1..s.len() - 2 {
//        for b in a + 1..s.len() - 1 {
//            for c in b + 1..s.len() {
//                if is_valid(0, a) && is_valid(a, b) && is_valid(b, c) && is_valid(c, s.len()) {
//                    let mut ip = String::with_capacity(s.len() + 3);
//                    ip.push_str(&s[0..a]);
//                    ip.push('.');
//                    ip.push_str(&s[a..b]);
//                    ip.push('.');
//                    ip.push_str(&s[b..c]);
//                    ip.push('.');
//                    ip.push_str(&s[c..s.len()]);
//                    result.push(ip);
//                }
//            }
//        }
//    }
//    result
//}

use std::cmp::min;

pub fn restore_ip_addresses(s: String) -> Vec<String> {
    if s.len() < 4 {
        return vec![];
    }

    let mut ctx = Context {
        segments: Vec::with_capacity(4),
        results: Vec::new(),
    };
    dfs(&mut ctx, &s, 3);
    ctx.results
}

struct Context<'a> {
    segments: Vec<&'a str>,
    results: Vec<String>,
}

fn dfs<'a>(ctx: &mut Context<'a>, string: &'a str, dots: usize) {
    if dots == 0 {
        if is_valid_segment(string) {
            ctx.segments.push(string);
            let result = ctx.segments.join(".");
            ctx.results.push(result);
            ctx.segments.pop().unwrap();
        }
    } else {
        for idx in 1..=min(3, string.len() - dots) {
            let segment = &string[..idx];
            if is_valid_segment(segment) {
                ctx.segments.push(segment);
                dfs(ctx, &string[idx..], dots - 1);
                ctx.segments.pop().unwrap();
            }
        }
    }
}

fn is_valid_segment(segment: &str) -> bool {
    match segment.parse::<u8>() {
        Err(_) => false,
        Ok(_) => {
            if segment.starts_with('0') {
                segment.len() == 1
            } else {
                true
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test() {
        let cases = vec![("25525511135", vec!["255.255.11.135", "255.255.111.35"])];
        for (input, expected) in cases {
            assert_eq!(set(restore_ip_addresses(input.to_string())), set(expected));
        }
    }

    fn set<S: ToString>(v: Vec<S>) -> HashSet<String> {
        let mut s = HashSet::with_capacity(v.len());
        s.extend(v.into_iter().map(|ts| ts.to_string()));
        s
    }
}
