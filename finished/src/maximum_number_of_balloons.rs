pub fn max_number_of_balloons<S>(text: S) -> i32
where
    S: AsRef<[u8]>,
{
    let mut vector = [0; 5];
    for ch in text.as_ref() {
        match *ch {
            b'a' => vector[0] += 1,
            b'b' => vector[1] += 1,
            b'l' => vector[2] += 1,
            b'n' => vector[3] += 1,
            b'o' => vector[4] += 1,
            _ => (),
        }
    }
    vector[2] /= 2;
    vector[4] /= 2;
    *vector.iter().min().unwrap()
}

#[test]
fn test() {
    let cases = vec![("nlaebolko", 1), ("loonbalxballpoon", 2), ("leetcode", 0)];
    for (text, expected) in cases {
        assert_eq!(max_number_of_balloons(text), expected);
    }
}
