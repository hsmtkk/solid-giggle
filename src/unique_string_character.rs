fn solve(a: &str, b: &str) -> String {
    let mut result = String::new();
    for ch in a.chars() {
        if !b.contains(ch) {
            result.push(ch);
        }
    }
    for ch in b.chars(){
        if !a.contains(ch){
            result.push(ch);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        assert_eq!(solve("xyab","xzca"),"ybzc".to_string());
        assert_eq!(solve("xyabb","xzca"),"ybbzc".to_string());
        assert_eq!(solve("abcd","xyz"),"abcdxyz".to_string());
        assert_eq!(solve("xxx","xzca"),"zca".to_string());
    }
}
