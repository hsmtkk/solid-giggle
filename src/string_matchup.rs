fn match_counts(a1: &[String], a2: &[String]) -> Vec<usize> {
    let counter = |s| a1.iter().filter(|&t| t == s).count();
    a2.iter().map(counter).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(
            match_counts(
                &[
                    "abc".to_string(),
                    "abc".to_string(),
                    "xyz".to_string(),
                    "abcd".to_string(),
                    "cde".to_string()
                ],
                &["abc".to_string(), "cde".to_string(), "uap".to_string()]
            ),
            vec![2, 1, 0]
        );
        assert_eq!(
            match_counts(
                &[
                    "abc".to_string(),
                    "xyz".to_string(),
                    "abc".to_string(),
                    "xyz".to_string(),
                    "cde".to_string()
                ],
                &["abc".to_string(), "cde".to_string(), "xyz".to_string()]
            ),
            vec![2, 1, 2]
        );
        assert_eq!(
            match_counts(
                &[
                    "quick".to_string(),
                    "brown".to_string(),
                    "fox".to_string(),
                    "is".to_string(),
                    "quick".to_string()
                ],
                &["quick".to_string(), "abc".to_string(), "fox".to_string()]
            ),
            vec![2, 0, 1]
        );
    }
}
