use std::collections::HashMap;

fn freq_seq(s: &str, sep: &str) -> String {
    let mut char_counter = HashMap::new();
    for ch in s.chars() {
        let counter = char_counter.entry(ch).or_insert(0);
        *counter += 1;
    }
    let mut result = Vec::new();
    for ch in s.chars() {
        let count = char_counter.get(&ch).unwrap();
        result.push(count.to_string());
    }
    result.join(sep)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(freq_seq("hello world", "-"), "1-1-3-3-2-1-1-2-1-3-1");
        assert_eq!(freq_seq("19999999", ":"), "1:7:7:7:7:7:7:7");
        assert_eq!(freq_seq("^^^**$", "x"), "3x3x3x2x2x1");
    }
}
