fn min_value(mut digits: Vec<i32>) -> i32 {
    digits.sort_unstable();
    digits.dedup();
    digits.iter().fold(0, |acc, n| acc * 10 + n)
}

#[cfg(test)]
mod tests {
    use super::min_value;

    #[test]
    fn basic_test() {
        assert_eq!(min_value(vec![1, 3, 1]), 13);
        assert_eq!(min_value(vec![4, 7, 5, 7]), 457);
        assert_eq!(min_value(vec![4, 8, 1, 4]), 148);
    }
}
