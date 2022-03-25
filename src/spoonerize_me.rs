fn spoonerize(words: &str) -> String {
    let ss: Vec<&str> = words.split(' ').collect();
    let first = ss[0];
    let second = ss[1];
    let mut result = String::new();
    result += &second[0..1];
    result += &first[1..];
    result += " ";
    result += &first[0..1];
    result += &second[1..];
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(spoonerize("nit picking"), "pit nicking");
        assert_eq!(spoonerize("wedding bells"), "bedding wells");
        assert_eq!(spoonerize("jelly beans"), "belly jeans");
    }
}
