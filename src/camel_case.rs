fn camel_case(sentence: &str) -> String {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    let capitalized_words: Vec<String> = words.iter().map(capitalize).collect();
    capitalized_words.join("")
}

fn capitalize(word: &&str) -> String {
    let head = &word[0..1];
    let tail = &word[1..];
    head.to_string().to_uppercase() + tail
}

#[cfg(test)]
mod tests {
    use super::camel_case;

    #[test]
    fn test_camel_case() {
        let test_cases = vec![
            ("test case", "TestCase"),
            ("camel case method", "CamelCaseMethod"),
            ("say hello ", "SayHello"),
            (" camel case word", "CamelCaseWord"),
            ("", ""),
        ];
        for (input, want) in test_cases {
            let got = camel_case(input);
            assert_eq!(want, got);
        }
    }
}
