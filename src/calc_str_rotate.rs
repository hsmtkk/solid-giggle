fn shifted_diff(first: &str, second: &str) -> Option<usize> {
    if first == second {
        return Some(0);
    }
    let str_len = first.len();
    let mut shifted = first.to_string();
    for i in 0..str_len{
        let last = shifted.chars().nth(str_len-1).unwrap();
        shifted = last.to_string() + &shifted[0..str_len-1];
        if shifted == second {
            return Some(i+1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(shifted_diff("eecoff", "coffee"), Some(4));
        assert_eq!(shifted_diff("Moose", "moose"), None);
        assert_eq!(shifted_diff("isn't", "'tisn"), Some(2));
        assert_eq!(shifted_diff("Esham", "Esham"), Some(0));
        assert_eq!(shifted_diff(" ", " "), Some(0));
        assert_eq!(shifted_diff("hoop", "pooh"), None);
        assert_eq!(shifted_diff("  ", " "), None);
    }
}
