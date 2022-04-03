fn shifter(s: &str) -> usize {
    if s.is_empty() {
        return 0;
    }
    let mut words: Vec<&str> = s.split(' ').collect();
    words.sort_unstable();
    words.dedup();
    words.iter().filter(is_shifter).count()
}

fn is_shifter(s: &&&str) -> bool {
    for ch in s.chars() {
        match ch {
            'H' | 'I' | 'N' | 'O' | 'S' | 'X' | 'Z' | 'M' | 'W' => {}
            _ => {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(shifter("ON"), 1, "Wrong result for input \"ON\"");
        assert_eq!(
            shifter("OS IS UPDATED"),
            2,
            "Wrong result for input \"OS IS UPDATED\""
        );
        assert_eq!(
            shifter("WHO IS WHO"),
            2,
            "Wrong result for input \"WHO IS WHO\""
        );
        assert_eq!(shifter("JS"), 0, "Wrong result for input \"JS\"");
        assert_eq!(
            shifter("I III I III"),
            2,
            "Wrong result for input \"I III I III\""
        );
        assert_eq!(shifter(""), 0, "Wrong result for input \"\"");
    }
}
