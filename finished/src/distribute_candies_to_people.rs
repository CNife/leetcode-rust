use std::cmp::min;

pub fn distribute_candies(mut candies: i32, num_people: i32) -> Vec<i32> {
    let num_people = num_people as usize;
    let mut n = 1;
    let mut i = 0;
    let mut res = vec![0; num_people];

    while candies > 0 {
        let this_n = min(candies, n);
        res[i] += this_n;
        candies -= this_n;
        n += 1;
        if i + 1 < num_people {
            i += 1;
        } else {
            i = 0;
        }
    }
    res
}

#[test]
fn test() {
    let cases = vec![(7, 4, vec![1, 2, 3, 1]), (10, 3, vec![5, 2, 3])];
    for (candies, num_people, expected) in cases {
        assert_eq!(distribute_candies(candies, num_people), expected);
    }
}
