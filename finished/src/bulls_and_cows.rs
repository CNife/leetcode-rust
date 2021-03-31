pub fn get_hint<S: AsRef<[u8]>>(secret: S, guess: S) -> String {
    let secret = secret.as_ref();
    let guess = guess.as_ref();
    let mut bull = 0;
    let mut buckets = [0; 10];

    for (s, g) in secret.iter().zip(guess.iter()) {
        if s == g {
            bull += 1;
        } else {
            buckets[(s - b'0') as usize] += 1;
            buckets[(g - b'0') as usize] -= 1;
        }
    }

    let positive_sum: i32 = buckets.iter().filter(|b| **b > 0).sum();
    let cow = secret.len() as i32 - bull - positive_sum;
    format!("{}A{}B", bull, cow)
}

#[test]
fn test() {
    let cases = vec![("1807", "7810", "1A3B"), ("1123", "0111", "1A1B")];
    for (secret, guess, expected) in cases {
        assert_eq!(get_hint(secret, guess), expected);
    }
}
