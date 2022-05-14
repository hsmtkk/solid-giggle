fn doubles(s: &str) -> String {
    let mut stack: Vec<char> = Vec::new();
    for ch in s.chars() {
        if stack.is_empty() {
            stack.push(ch);
        } else {
            let prev = stack.pop().unwrap();
            if ch != prev {
                stack.push(prev);
                stack.push(ch);
            }
        }
    }
    stack.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::doubles;

    #[test]
    fn sample_tests() {
        assert_eq!(doubles(&"abbbzz"), "ab");
        assert_eq!(doubles(&"zzzzykkkd"), "ykd");
        assert_eq!(doubles(&"abbcccdddda"), "aca");
        assert_eq!(doubles(&"vvvvvoiiiiin"), "voin");
        assert_eq!(doubles(&"rrrmooomqqqqj"), "rmomj");
        assert_eq!(doubles(&"xxbnnnnnyaaaaam"), "bnyam");
    }
}
