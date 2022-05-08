fn parity(ns: &[i32]) -> i32 {
    for n in ns {
        let m = -1 * n;
        if !ns.contains(&m) {
            return *n;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::parity;

    #[test]
    fn test_parity() {
        let test_cases = vec![
            (vec![1, -1, 2, -2, 3], 3),
            (vec![-3, 1, 2, 3, -1, -4, -2], -4),
            (vec![1, -1, 2, -2, 3, 3], 3),
            (vec![-110, 110, -38, -38, -62, 62, -38, -38, -38], -38),
            (vec![-9, -105, -9, -9, -9, -9, 105], -9),
        ];
        for test_case in test_cases {
            let got = parity(&test_case.0);
            assert_eq!(test_case.1, got);
        }
    }
}
