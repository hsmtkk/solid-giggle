fn get_middle(s: &str) -> &str {
    let i = s.len() / 2;
    if s.len() % 2 == 0 {
        &s[i - 1..i + 1]
    } else {
        &s[i..i + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::get_middle;
    #[test]
    fn example_tests() {
        assert_eq!(get_middle("test"), "es");
        assert_eq!(get_middle("testing"), "t");
        assert_eq!(get_middle("middle"), "dd");
        assert_eq!(get_middle("A"), "A");
        assert_eq!(get_middle("of"), "of");
    }
}
