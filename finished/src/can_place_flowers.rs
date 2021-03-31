pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
    if flowerbed.is_empty() {
        return false;
    }

    let mut m = 0;
    flowerbed.push(0);
    if flowerbed[0] == 0 && flowerbed[1] == 0 {
        flowerbed[0] = 1;
        m += 1;
    }

    for i in 1..flowerbed.len() - 1 {
        if flowerbed[i - 1] == 0 && flowerbed[i] == 0 && flowerbed[i + 1] == 0 {
            flowerbed[i] = 1;
            m += 1;
            if m >= n {
                return true;
            }
        }
    }
    m >= n
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 0, 0, 0, 1], 1, true),
        (vec![1, 0, 0, 0, 1], 2, false),
        (vec![1, 0, 0, 0, 0, 1], 2, false),
    ];
    for (flowerbed, n, expected) in cases {
        assert_eq!(can_place_flowers(flowerbed, n), expected);
    }
}
