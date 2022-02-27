fn solution(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    for i in 0..s.len()/2{
        let j = i * 2;
        let first = s.chars().nth(j).unwrap();
        let second = s.chars().nth(j+1).unwrap();
        let mut t = String::new();
        t.push(first);
        t.push(second);
        result.push(t);
    }
    if s.len() % 2 == 1 {
        let last = s.chars().nth(s.len()-1).unwrap();
        let mut t = String::new();
        t.push(last);
        t.push('_');
        result.push(t);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solution("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(solution(""), [] as [&str; 0]);
    }
}

