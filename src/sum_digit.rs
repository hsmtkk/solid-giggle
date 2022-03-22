fn digit_root(n:u32) -> u32 {
    if n < 10 {
        return n;
    }
    let m = n.to_string().chars().map(|ch| ch.to_digit(10).unwrap()).sum();
    digit_root(m)
}

#[cfg(test)]
mod tests {
    use super::digit_root;

    struct TestCase {
        input: u32,
        want: u32,
    }

    impl TestCase {
        fn new(input:u32, want:u32) -> Self {
            TestCase{input, want}
        }
    }

    #[test]
    fn test(){
        let test_cases = vec![
            TestCase::new(16, 7),
            TestCase::new(195, 6),
            TestCase::new(992, 2),
            TestCase::new(167346, 9),
            TestCase::new(0, 0),
        ];
        for test_case in test_cases {
            assert_eq!(test_case.want, digit_root(test_case.input));
        }
    }
}
