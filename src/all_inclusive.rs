fn contain_all_rotations(s: &str, ss: &[&str]) -> bool {
    let rotations = all_rotations(s);
    for rot in rotations {
        let r: &str = &rot;
        if !ss.contains(&r) {
            return false;
        }
    }
    true
}

fn all_rotations(s: &str) -> Vec<String> {
    let mut results = Vec::new();
    for i in 0..s.len() {
        let mut rotated: String = String::new();
        rotated += &s[i..];
        rotated += &s[..i];
        results.push(rotated);
    }
    results
}

#[cfg(test)]
mod tests {
    use super::contain_all_rotations;

    #[test]
    fn test0() {
        assert!(contain_all_rotations(
            "bsjq",
            &vec!["bsjq", "qbsj", "sjqb", "twZNsslC", "jqbs"]
        ))
    }

    #[test]
    fn test1() {
        assert!(!contain_all_rotations(
            "XjYABhR",
            &vec![
                "TzYxlgfnhf",
                "yqVAuoLjMLy",
                "BhRXjYA",
                "YABhRXj",
                "hRXjYAB",
                "jYABhRX",
                "XjYABhR",
                "ABhRXjY"
            ]
        ))
    }

    #[test]
    fn test2() {
        assert!(contain_all_rotations("", &vec![]));
    }
}
