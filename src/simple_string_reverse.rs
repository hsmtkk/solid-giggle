fn solve(s: &str) -> String {
    let space_trimmed: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    let mut reversed: String = space_trimmed.chars().rev().collect();
    for i in space_indexes(s) {
        reversed.insert(i, ' ');
    }
    reversed
}

fn space_indexes(s: &str) -> Vec<usize> {
    let mut indexes = Vec::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_whitespace() {
            indexes.push(i);
        }
    }
    indexes
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn basic_tests() {
        assert_eq!(solve(&"codewars"), "srawedoc");
        assert_eq!(solve(&"your code"), "edoc ruoy");
        assert_eq!(solve(&"your code rocks"), "skco redo cruoy");
        assert_eq!(solve(&"i love codewars"), "s rawe docevoli");
    }
}
