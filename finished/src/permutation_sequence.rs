pub fn get_permutation(n: i32, k: i32) -> String {
    let mut buf: Vec<u8> = (0..n).map(|num| num as u8 + b'1').collect();
    for _ in 1..k {
        match (0..buf.len() - 1).rfind(|&i| buf[i] < buf[i + 1]) {
            None => {
                buf.reverse();
            }
            Some(m) => {
                let n = (m + 1..buf.len()).rfind(|&i| buf[i] > buf[m]).unwrap();
                buf.swap(m, n);
                buf[m + 1..].reverse();
            }
        }
    }
    unsafe { String::from_utf8_unchecked(buf) }
}

#[test]
fn test() {
    let cases = vec![(3, 3, "213"), (4, 9, "2314")];
    for (n, k, expected) in cases {
        assert_eq!(get_permutation(n, k), expected);
    }
}
