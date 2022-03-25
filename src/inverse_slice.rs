fn inverse_slice<T: Clone>(input: &[T], a: usize, b: usize) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();
    if a <= input.len(){
        for x in &input[..a]{
            result.push(x.clone());
        }
    }
    if b <= input.len(){
        for x in &input[b..]{
            result.push(x.clone());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(inverse_slice(&[12, 14, 63, 72, 55, 24], 2, 4), [12, 14, 55, 24]);
        assert_eq!(inverse_slice(&[12, 14, 63, 72, 55, 24], 0, 3), [72, 55, 24]);
        assert_eq!(
            inverse_slice(&["Intuition", "is", "a", "poor", "guide", "when", "facing", "probabilistic", "evidence"], 5, 13),
            ["Intuition", "is", "a", "poor", "guide"]);
        assert_eq!(inverse_slice::<i32>(&[], 1, 4), []);
        assert_eq!(inverse_slice(&[0, 0, 0, 1, 0], 0, 3), [1, 0]);
    }
}
