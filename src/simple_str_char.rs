fn count_char(s: &str) -> (usize, usize, usize, usize) {
    let mut upper = 0;
    let mut lower = 0;
    let mut number = 0;
    let mut special = 0;
    for ch in s.chars() {
        if 'a' <= ch && ch <= 'z' {
            lower += 1;
        } else if 'A' <= ch && ch <= 'Z' {
            upper += 1;
        } else if '0' <= ch && ch <= '9' {
            number += 1;
        } else {
            special += 1;
        }
    }
    (upper, lower, number, special)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test0() {
        let test_cases = vec![
            ("Codewars@codewars123.com", (1, 18, 3, 2)),
            ("bgA5<1d-tOwUZTS8yQ", (7, 6, 3, 2)),
            ("P*K4%>mQUDaG$h=cx2?.Czt7!Zn16p@5H", (9, 9, 6, 9)),
            ("RYT'>s&gO-.CM9AKeH?,5317tWGpS<*x2ukXZD", (15, 8, 6, 9)),
            ("$Cnl)Sr<7bBW-&qLHI!mY41ODe", (10, 7, 3, 6)),
        ];
        for test_case in test_cases {
            let got = super::count_char(test_case.0);
            assert_eq!(test_case.1, got);
        }
    }
}
