use std::cmp::max;

pub fn longest_substring(s: String, k: i32) -> i32 {
    if k < 2 {
        s.len() as i32
    } else {
        divide_and_conquer(s.as_bytes(), k)
    }
}

fn divide_and_conquer(input: &[u8], k: i32) -> i32 {
    let n = input.len() as i32;
    if n < k {
        return 0;
    }

    let mut counts = [0; 26];
    for ch in input {
        counts[(ch - b'a') as usize] += 1;
    }

    for (i, ch) in input.iter().enumerate() {
        let count = counts[(ch - b'a') as usize];
        if count < k {
            return max(
                divide_and_conquer(&input[..i], k),
                divide_and_conquer(&input[i + 1..], k),
            );
        }
    }
    n
}

#[test]
fn test() {
    let cases = vec![("aaabb", 3, 3), ("ababbc", 2, 5)];
    for (input, k, expected) in cases {
        assert_eq!(longest_substring(input.to_string(), k), expected);
    }
}
