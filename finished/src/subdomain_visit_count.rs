use std::collections::HashMap;

pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
    let mut domain_counts = HashMap::new();
    for cpdomain in &cpdomains {
        let space_idx = cpdomain.find(' ').unwrap();
        let count = &cpdomain[..space_idx].parse::<usize>().unwrap();
        let mut domain = &cpdomain[space_idx + 1..];

        while !domain.is_empty() {
            *domain_counts.entry(domain).or_insert(0) += count;
            domain = domain.find('.').map_or("", |idx| &domain[idx + 1..]);
        }
    }
    domain_counts
        .into_iter()
        .map(|(domain, count)| format!("{} {}", count, domain))
        .collect()
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use std::iter::FromIterator;

    use super::*;

    fn cases() -> Vec<(Vec<&'static str>, Vec<&'static str>)> {
        vec![
            (
                vec!["9001 discuss.leetcode.com"],
                vec!["9001 discuss.leetcode.com", "9001 leetcode.com", "9001 com"],
            ),
            (
                vec![
                    "900 google.mail.com",
                    "50 yahoo.com",
                    "1 intel.mail.com",
                    "5 wiki.org",
                ],
                vec![
                    "901 mail.com",
                    "50 yahoo.com",
                    "900 google.mail.com",
                    "5 wiki.org",
                    "5 org",
                    "1 intel.mail.com",
                    "951 com",
                ],
            ),
        ]
    }

    fn string(strings: Vec<&'static str>) -> Vec<String> {
        strings.into_iter().map(|s| s.to_string()).collect()
    }

    fn hash_set(strings: Vec<String>) -> HashSet<String> {
        HashSet::from_iter(strings.into_iter())
    }

    #[test]
    fn test_subdomain_visits() {
        for (input, expected) in cases() {
            let output = hash_set(subdomain_visits(string(input)));
            let expected = hash_set(string(expected));
            assert_eq!(output, expected);
        }
    }
}
