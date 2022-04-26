fn alternate_capitalize(s: &str) -> (String, String) {
    (even_capitalize(s), odd_capitalize(s))
}

fn even_capitalize(s: &str) -> String {
    let mut result = String::new();
    for (i, t) in s.chars().enumerate() {
        if i % 2 == 0 {
            result += &t.to_uppercase().to_string();
        } else {
            result.push(t);
        }
    }
    result
}

fn odd_capitalize(s: &str) -> String {
    let mut result = String::new();
    for (i, t) in s.chars().enumerate() {
        if i % 2 == 1 {
            result += &t.to_uppercase().to_string();
        } else {
            result.push(t);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::alternate_capitalize;

    #[test]
    fn test_alternate_capitalize() {
        let test_cases = vec![
            ("abcdef", "AbCdEf", "aBcDeF"),
            ("codewars", "CoDeWaRs", "cOdEwArS"),
            ("abracadabra", "AbRaCaDaBrA", "aBrAcAdAbRa"),
            ("codewarriors", "CoDeWaRrIoRs", "cOdEwArRiOrS"),
            ("indexinglessons", "InDeXiNgLeSsOnS", "iNdExInGlEsSoNs"),
            (
                "codingisafunactivity",
                "CoDiNgIsAfUnAcTiViTy",
                "cOdInGiSaFuNaCtIvItY",
            ),
        ];
        for test_case in test_cases {
            let input = test_case.0;
            let want = (test_case.1.to_string(), test_case.2.to_string());
            let got = alternate_capitalize(test_case.0);
            assert_eq!(want, got);
        }
    }
}
