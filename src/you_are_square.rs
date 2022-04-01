fn is_square_number(n: i64) -> bool {
    if n < 0 {
        return false;
    }
    let sqrt = (n as f64).sqrt();
    let floor = sqrt.floor() as i64;
    if n == floor * floor {
        return true;
    }
    let ceil = sqrt.ceil() as i64;
    if n == ceil * ceil {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::is_square_number;

    #[test]
    fn test_is_square_number() {
        let test_cases = vec![
            (-1, false),
            (0, true),
            (3, false),
            (4, true),
            (25, true),
            (26, false),
        ];
        for (input, want) in test_cases {
            let got = is_square_number(input);
            assert_eq!(want, got);
        }
    }
}
