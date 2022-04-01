fn max_len_diff(s1: &[&str], s2: &[&str]) -> Option<usize> {
    if s1.is_empty() || s2.is_empty() {
        return None;
    }
    let s1_max = s1.iter().map(|s| s.len()).max().unwrap();
    let s2_max = s2.iter().map(|s| s.len()).max().unwrap();
    let s1_min = s1.iter().map(|s| s.len()).min().unwrap();
    let s2_min = s2.iter().map(|s| s.len()).min().unwrap();
    let abs1 = abs_diff(s1_max, s2_min);
    let abs2 = abs_diff(s2_max, s1_min);
    if abs1 > abs2 {
        Some(abs1)
    } else {
        Some(abs2)
    }
}

fn abs_diff(m: usize, n: usize) -> usize {
    if m > n {
        m - n
    } else {
        n - m
    }
}

#[cfg(test)]
mod tests {
    use super::max_len_diff;

    #[test]
    fn test0() {
        let s1 = vec![
            "hoqq",
            "bbllkw",
            "oox",
            "ejjuyyy",
            "plmiis",
            "xxxzgpsssa",
            "xxwwkktt",
            "znnnnfqknaz",
            "qqquuhii",
            "dvvvwz",
        ];
        let s2 = vec!["cccooommaaqqoxii", "gggqaffhhh", "tttoowwwmmww"];
        let want = Some(13);
        let got = max_len_diff(&s1, &s2);
        assert_eq!(want, got);
    }

    fn test1() {
        let s1 = vec![
            "ejjjjmmtthh",
            "zxxuueeg",
            "aanlljrrrxx",
            "dqqqaaabbb",
            "oocccffuucccjjjkkkjyyyeehh",
        ];
        let s2 = vec!["bbbaaayddqbbrrrv"];
        let want = Some(10);
        let got = max_len_diff(&s1, &s2);
        assert_eq!(want, got);
    }

    fn test2() {
        let s1 = vec!["cccooommaaqqoxii", "gggqaffhhh", "tttoowwwmmww"];
        let s2 = vec![];
        let want = None;
        let got = max_len_diff(&s1, &s2);
        assert_eq!(want, got);
    }
}
