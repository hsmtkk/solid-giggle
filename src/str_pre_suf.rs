fn longest_prefix_suffix(s: &str) -> usize {
    for i in (0..s.len() / 2 + 1).rev() {
        let prefix = &s[..i];
        let suffix = &s[s.len() - i..];
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
            ("abcd", 0, "abcd"),
            ("abcda", 1, "abcda"),
            ("abcdabc", 3, "abcdabc"),
            ("abcabc", 3, "abcabc"),
            ("abcabca", 1, "abcabca"),
            ("abcdabcc", 0, "abcdabcc"),
            ("aaaaa", 2, "aaaaa"),
            ("aaaa", 2, "aaaa"),
            ("aaa", 1, "aaa"),
            ("aa", 1, "aa"),
            ("a", 0, "a"),
            ("acbacc", 0, "acbacc"),
        ];
        for test_case in test_cases {
            let got = longest_prefix_suffix(test_case.0);
            assert_eq!(test_case.1, got, "{}", test_case.2);
        }
    }
}
