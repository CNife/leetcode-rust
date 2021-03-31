use std::collections::HashMap;

pub fn num_tile_possibilities(tiles: String) -> i32 {
    let mut char_map = HashMap::new();
    for ch in tiles.chars() {
        *char_map.entry(ch).or_insert(0i32) += 1;
    }
    let mut counts: Vec<i32> = char_map.values().cloned().collect();
    let mut result = 0;
    backtrace(&mut counts, &mut result);
    result
}

fn backtrace(counts: &mut Vec<i32>, result: &mut i32) {
    for i in 0..counts.len() {
        if counts[i] > 0 {
            counts[i] -= 1;
            *result += 1;
            backtrace(counts, result);
            counts[i] += 1;
        }
    }
}

#[test]
fn test() {
    let cases = vec![("AAB", 8), ("AAABBC", 188)];
    for (tiles, expected) in cases {
        assert_eq!(num_tile_possibilities(tiles.to_string()), expected);
    }
}
