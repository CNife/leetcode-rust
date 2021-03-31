pub struct Solution;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut res = Vec::with_capacity(2i32.pow(n as u32) as usize);
        res.push(0);
        let mut head = 1;
        for _ in 0..n as usize {
            for j in (0..res.len()).rev() {
                res.push(head + res[j]);
            }
            head <<= 1;
        }
        res
    }
}

#[test]
fn test_gray_code() {
    let is_gray_code = |vec: Vec<i32>| {
        vec[0] == 0
            && (0..vec.len() - 1).all(|i| {
                let diff = vec[i] ^ vec[i + 1];
                diff.count_ones() == 1
            })
    };

    let inputs = vec![0, 3, 10];
    for n in inputs {
        assert!(is_gray_code(Solution::gray_code(n)));
    }
}
