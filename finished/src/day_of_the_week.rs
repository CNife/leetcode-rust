pub struct Solution;

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let days_in_month = if Solution::is_leap_year(year) {
            LEAP_YEAR_MONTHS[month as usize]
        } else {
            NON_LEAP_YEAR_MONTHS[month as usize]
        };
        let days_in_year: i32 = (1970..year)
            .map(|y| if Solution::is_leap_year(y) { 366 } else { 365 })
            .sum();
        let days = day + days_in_month + days_in_year;
        DAY_IN_WEEK[(days % 7) as usize].to_string()
    }

    #[inline]
    fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
}

const LEAP_YEAR_MONTHS: [i32; 13] = [
    std::i32::MIN, // for padding
    0,
    31,
    60,
    91,
    121,
    152,
    182,
    213,
    244,
    274,
    305,
    335,
];

const NON_LEAP_YEAR_MONTHS: [i32; 13] = [
    std::i32::MIN, // for padding
    0,
    31,
    59,
    90,
    120,
    151,
    181,
    212,
    243,
    273,
    304,
    334,
];

const DAY_IN_WEEK: [&str; 7] = [
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
    "Monday",
    "Tuesday",
];

#[test]
fn test_day_of_the_week() {
    let cases = vec![
        (31, 8, 2019, "Saturday"),
        (18, 7, 1999, "Sunday"),
        (15, 8, 1993, "Sunday"),
    ];
    for (day, month, year, expected) in cases {
        let output = Solution::day_of_the_week(day, month, year);
        assert_eq!(output, expected.to_string());
    }
}
