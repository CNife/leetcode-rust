pub fn ways_to_change(n: i32) -> i32 {
    let n = n as usize;
    let mut f = vec![0; n + 1];
    f[0] = 1;
    for &coin in &COINS {
        for i in coin..=n {
            f[i] = (f[i] + f[i - coin]) % MOD;
        }
    }
    f[n]
}

const COINS: [usize; 4] = [25, 10, 5, 1];
const MOD: i32 = 1000000007;

#[test]
fn test() {
    let cases = vec![/*(5, 2),*/ (10, 4)];
    for (n, expect) in cases {
        assert_eq!(ways_to_change(n), expect);
    }
}
