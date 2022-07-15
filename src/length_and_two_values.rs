fn alternate<'a>(n: usize, first_value: &'a str, second_value: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for i in 0..n {
        if i % 2 == 0 {
            result.push(first_value);
        } else {
            result.push(second_value);
        }
    }
    result
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::alternate;

    #[test]
    fn sample_tests() {
        assert_eq!(
            alternate(5, "true", "false"),
            ["true", "false", "true", "false", "true"]
        );
        assert_eq!(
            alternate(20, "blue", "red"),
            [
                "blue", "red", "blue", "red", "blue", "red", "blue", "red", "blue", "red", "blue",
                "red", "blue", "red", "blue", "red", "blue", "red", "blue", "red"
            ]
        );
        let empty_vec: Vec<String> = Vec::new();
        assert_eq!(alternate(0, "lemons", "apples"), empty_vec);
    }
}
