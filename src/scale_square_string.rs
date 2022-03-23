fn scale(s: &str, k: u32, n: u32) -> String {
    if s.is_empty() {
        return "".to_string();
    }
    let mut rows: Vec<String> = Vec::new();
    let lines: Vec<&str> = s.split('\n').collect();
    for line in lines {
        let mut chars: Vec<char> = Vec::new();
        for ch in line.chars() {
            for _i in 0..k {
                chars.push(ch);
            }
        }
        let row: String = chars.into_iter().collect();
        for _i in 0..n {
            rows.push(row.clone());
        }
    }
    rows.join("\n")
}

#[cfg(test)]
mod tests {
    use super::scale;

    #[test]
    fn test0() {
        let input = "abcd\nefgh\nijkl\nmnop";
        let want = "aabbccdd\naabbccdd\naabbccdd\neeffgghh\neeffgghh\neeffgghh\niijjkkll\niijjkkll\niijjkkll\nmmnnoopp\nmmnnoopp\nmmnnoopp";
        let got = scale(input, 2, 3);
        assert_eq!(want, got);
    }

    #[test]
    fn test1() {
        let input = "";
        let want = "";
        let got = scale(input, 5, 5);
        assert_eq!(want, got);
    }

    #[test]
    fn test2() {
        let input = "Kj\nSH";
        let want = "Kj\nKj\nSH\nSH";
        let got = scale(input, 1, 2);
        assert_eq!(want, got);
    }
}
