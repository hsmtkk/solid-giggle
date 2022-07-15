fn switcher(numbers: Vec<&str>) -> String {
    numbers.iter().map(num_char).collect()
}

fn num_char(digit: &&str) -> char {
    let n: u8 = digit.parse().unwrap();
    match n {
        27 => '!',
        28 => '?',
        29 => ' ',
        _ => {
            let ch = b'a' + (26 - n);
            ch as char
        }
    }
}

#[cfg(test)]
mod tests {
    use super::switcher;
    #[test]
    fn test_switcher() {
        assert_eq!(
            switcher(vec!["24", "12", "23", "22", "4", "26", "9", "8"]),
            "codewars"
        );
        assert_eq!(
            switcher(vec![
                "25", "7", "8", "4", "14", "23", "8", "25", "23", "29", "16", "16", "4"
            ]),
            "btswmdsbd kkw"
        );
        assert_eq!(switcher(vec!["4", "24"]), "wc");
    }
}
