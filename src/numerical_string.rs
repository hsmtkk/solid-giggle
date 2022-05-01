use std::collections::HashMap;

fn numerical_string(s: &str) -> String {
    let mut counter = HashMap::new();
    let mut result = String::new();
    for ch in s.chars() {
        counter.entry(ch).and_modify(|c| *c += 1).or_insert(1);
        let count = counter.get(&ch).unwrap();
        result += &count.to_string();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::numerical_string;

    #[test]
    fn test_numerical_string() {
        let test_cases = vec![
            ("Hello, World!", "1112111121311"),
            ("Inconceivable!", "11112211111121"),
            ("hello hello", "11121122342"),
            ("Hello", "11121"),
            ("aaaaaaaaaaaa", "123456789101112"),
        ];
        for test_case in test_cases {
            let input = test_case.0;
            let want = test_case.1;
            let got = numerical_string(input);
            assert_eq!(want, got);
        }
    }
}

/*
        {"Hello, World!", "1112111121311"},
        {"Inconceivable!", "11112211111121"},
        {"hello hello", "11121122342"},
        {"Hello", "11121"},
        {"aaaaaaaaaaaa", "123456789101112"},
*/
