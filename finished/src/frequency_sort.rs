use std::collections::{BinaryHeap, HashMap};

pub fn frequency_sort(s: String) -> String {
    let mut buf = s.into_bytes();

    let mut cnt_map = HashMap::new();
    for b in buf.drain(..) {
        *cnt_map.entry(b).or_insert(0) += 1;
    }

    let mut cnt_heap = BinaryHeap::with_capacity(cnt_map.len());
    for (b, cnt) in cnt_map {
        cnt_heap.push((cnt, b));
    }

    while let Some((cnt, b)) = cnt_heap.pop() {
        for _ in 0..cnt {
            buf.push(b);
        }
    }

    unsafe { String::from_utf8_unchecked(buf) }
}

#[test]
fn test_frequency_sort() {
    frequency_sort("2a554442f544asfasssffffasss".to_string());
}
