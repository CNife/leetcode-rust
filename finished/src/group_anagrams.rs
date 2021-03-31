pub fn group_anagrams(strings: Vec<String>) -> Vec<Vec<String>> {
    const PRIMES: [u32; 26] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
        101, 103,
    ];

    let mut indexes: Vec<u32> = Vec::new();
    let mut buckets: Vec<Vec<String>> = Vec::new();
    for string in strings {
        let key = string
            .bytes()
            .fold(1, |product, ch| product * PRIMES[(ch - b'a') as usize]);
        match indexes.binary_search(&key) {
            Ok(index) => {
                buckets[index].push(string);
            }
            Err(insert_index) => {
                indexes.insert(insert_index, key);
                buckets.insert(insert_index, vec![string]);
            }
        }
    }
    buckets
}

#[test]
fn test() {
    use utils::{assert_same_set, vec_of};

    let tests = vec![(
        vec!["eat", "tea", "tan", "ate", "nat", "bat"],
        vec![vec!["ate", "eat", "tea"], vec!["nat", "tan"], vec!["bat"]],
    )];
    for (strings, want) in tests {
        let mut output = group_anagrams(vec_of(strings));
        output.iter_mut().for_each(|vs| vs.sort_unstable());
        let want = want
            .into_iter()
            .map(|mut vs| {
                vs.sort_unstable();
                vec_of(vs)
            })
            .collect();
        assert_same_set(output, want);
    }
}
