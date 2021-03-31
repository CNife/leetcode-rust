pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut bitmap = BitMap::new(nums.len() - 1);
    for num in nums {
        if bitmap.get(num) {
            return num;
        } else {
            bitmap.set(num, true);
        }
    }
    unreachable!()
}

#[derive(Debug)]
struct BitMap(Vec<u64>);

impl BitMap {
    fn new(width: usize) -> BitMap {
        let chunks = (width / 64) as usize + 1;
        BitMap(vec![0; chunks])
    }

    fn get(&self, index: i32) -> bool {
        let (chunk, mask) = self.make_index(index);
        self.0[chunk] & mask != 0
    }

    fn set(&mut self, index: i32, flag: bool) {
        let (chunk, mask) = self.make_index(index);
        if flag {
            self.0[chunk] |= mask;
        } else {
            self.0[chunk] &= !mask;
        }
    }

    fn make_index(&self, index: i32) -> (usize, u64) {
        let chunk = (index / 64) as usize;
        let mask = 1u64 << (index % 64);
        (chunk, mask)
    }
}

#[test]
fn test() {
    let cases = vec![(vec![1, 3, 4, 2, 2], 2), (vec![3, 1, 3, 4, 2], 3)];
    for (nums, expect) in cases {
        assert_eq!(find_duplicate(nums), expect);
    }
}
