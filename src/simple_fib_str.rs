fn simple_fib_str(n: u32) -> String {
    match n {
        0 => "0".to_string(),
        1 => "01".to_string(),
        _ => {
            let mut s = simple_fib_str(n - 1);
            s += &simple_fib_str(n - 2);
            s
        }
    }
}

#[cfg(test)]
mod tests {
    use super::simple_fib_str;

    #[test]
    fn test_simple_fib_str() {
        let test_cases = vec![
            (0, "0"),
            (1, "01"),
            (2, "010"),
            (3, "01001"),
            (5, "0100101001001"),
        ];
        for (input, want) in test_cases {
            let got = simple_fib_str(input);
            assert_eq!(want, got);
        }
    }
}
