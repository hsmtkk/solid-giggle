fn num_of_div(n: u32, divisor: u32) -> u32 {
    let mut n = n;
    let mut div = 0;
    loop {
        if n < divisor {
            return div;
        }
        n /= divisor;
        div += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::num_of_div;

    #[test]
    fn test_num_of_div() {
        let test_cases = vec![
            (6, 2, 2),
            (100, 2, 6),
            (2450, 5, 4),
            (9999, 3, 8),
            (2, 3, 0),
            (5, 5, 1),
        ];
        for (n, divisor, want) in test_cases {
            assert_eq!(want, num_of_div(n, divisor));
        }
    }
}
