fn modified_sum(array: &[i32], power: u32) -> i32 {
    let first = array.iter().map(|x| x.pow(power)).fold(0, |acc, x| acc + x);
    let second = array.iter().fold(0, |acc, x| acc + x);
    first - second
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(modified_sum(&vec![1, 2, 3], 3), 30);
        assert_eq!(modified_sum(&vec![1, 2], 5), 30);
    }
}
