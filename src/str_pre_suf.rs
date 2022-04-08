fn longest_prefix_suffix(s: &str) -> usize {
    for i in (0..s.len()).rev() {
        let prefix = &s[..i];
        let suffix = &s[s.len()-i..];
        if prefix == suffix {
            return i;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::longest_prefix_suffix;

    #[test]
    fn test_longest_prefix_suffix() {
        let test_cases = vec![
            ("abcd",     0),
        ("abcda":    1),
        ("abcdabc":  3),
        ("abcabc":   3),
        ("abcabca":  1),
        ("abcdabcc": 0),
        ("aaaaa":    2),
        ("aaaa":     2),
        ("aaa":      1),
        ("aa":       1),
        ("a":        0),
        ("acbacc":   0),
        ];
        for test_case in test_cases {
            let got = longest_prefix_suffix(test_case.0);
            assert_eq!(test_case.1, got);
        }
    }
}
