use std::cmp::{max, Ordering};
use std::str::FromStr;

pub fn compare_version(version1: String, version2: String) -> i32 {
    let v1: Version = version1.parse().unwrap();
    let v2: Version = version2.parse().unwrap();
    match v1.cmp(&v2) {
        Ordering::Equal => 0,
        Ordering::Greater => 1,
        Ordering::Less => -1,
    }
}

#[derive(Debug)]
struct Version(Vec<i32>);

impl FromStr for Version {
    type Err = <i32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = Vec::new();
        for part in s.split('.') {
            let num = part.parse()?;
            v.push(num);
        }
        Ok(Version(v))
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in 0..max(self.0.len(), other.0.len()) {
            let lhs = self.0.get(i).unwrap_or(&0);
            let rhs = other.0.get(i).unwrap_or(&0);
            match lhs.cmp(rhs) {
                Ordering::Equal => {}
                order => return order,
            }
        }
        Ordering::Equal
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        match self.cmp(other) {
            Ordering::Equal => true,
            _ => false,
        }
    }
}

impl Eq for Version {}

#[test]
fn test() {
    let cases = vec![
        ("0.1", "1.1", -1),
        ("1.0.1", "1", 1),
        ("7.5.2.4", "7.5.3", -1),
        ("1.01", "1.001", 0),
        ("1.0", "1.0.0", 0),
    ];
    for (v1, v2, expect) in cases {
        assert_eq!(compare_version(v1.into(), v2.into()), expect);
    }
}
