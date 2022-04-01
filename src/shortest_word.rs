fn find_shortest_word(line: &str) -> usize {
    let words: Vec<&str> = line.split(' ').collect();
    words.iter().map(|w| w.len()).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::find_shortest_word;

    #[test]
    fn test_find_shortest_word() {
        let test_cases = vec![
            ("bitcoin take over the world maybe who knows perhaps", 3),
            (
                "turns out random test cases are easier than writing out basic ones",
                3,
            ),
            ("Let's travel abroad shall we", 2),
        ];
        for (input, want) in test_cases {
            let got = find_shortest_word(input);
            assert_eq!(want, got);
        }
    }
}
