fn create_phone_number(numbers: &[u8]) -> String {
    let first = format!("{}{}{}", numbers[0], numbers[1], numbers[2]);
    let second = format!("{}{}{}", numbers[3], numbers[4], numbers[5]);
    let third = format!("{}{}{}{}", numbers[6], numbers[7], numbers[8], numbers[9]);
    format!("({}) {}-{}", first, second, third)
}

#[cfg(test)]
mod tests {
    use super::create_phone_number;

    #[test]
    fn returns_expected() {
        assert_eq!(
            create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
            "(123) 456-7890"
        );
        assert_eq!(
            create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            "(111) 111-1111"
        );
        assert_eq!(
            create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]),
            "(123) 456-7899"
        );
    }
}
