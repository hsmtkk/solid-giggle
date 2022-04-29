fn index_capitalize(s: &str, indexes: &[usize]) -> String {
    let mut result = String::new();
    for (index, s) in s.chars().enumerate() {
        let t: char = if indexes.contains(&index) {
            s.to_ascii_uppercase()
        } else {
            s
        };
        result.push(t);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::index_capitalize;

    #[test]
    fn test_index_capitalize() {
        let test_cases = vec![
            ("abcdef", vec![1, 2, 5], "aBCdeF"),
            ("abcdef", vec![1, 2, 5, 100], "aBCdeF"),
            ("codewars", vec![1, 3, 5, 50], "cOdEwArs"),
            ("abracadabra", vec![2, 6, 9, 10], "abRacaDabRA"),
            ("codewarriors", vec![5], "codewArriors"),
            ("indexinglessons", vec![0], "Indexinglessons"),
        ];
        for test_case in test_cases {
            let input = test_case.0;
            let indexes = test_case.1;
            let want = test_case.2;
            let got = index_capitalize(input, &indexes);
            assert_eq!(want, got);
        }
    }
}
