use std::collections::HashMap;

fn has_unique_chars(s: &str) -> bool {
    let mut seen = HashMap::new();
    for ch in s.chars() {
        match seen.get(&ch) {
            Some(_found) => {
                return false;
            }
            None => {
                seen.insert(ch, true);
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::has_unique_chars;
    #[test]
    fn test0() {
        let test_cases = vec![
            ("  nAa", false),
            ("abcde", true),
            ("++-", false),
            ("AaBbC", true),
        ];
        for (input, want) in test_cases {
            let got = has_unique_chars(input);
            assert_eq!(want, got);
        }
    }
}
