fn digitize(n: u64) -> Vec<u8> {
    let ds = n.to_string();
    let mut ds: Vec<char> = ds.chars().collect();
    ds.reverse();
    ds.iter().map(|s| s.to_digit(10).unwrap() as u8).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(digitize(35231), vec![1, 3, 2, 5, 3]);
        assert_eq!(digitize(0), vec![0]);
    }
}
