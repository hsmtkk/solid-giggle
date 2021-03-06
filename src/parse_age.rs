fn get_age(age: &str) -> u32 {
    let age_digit: &str = age.split(' ').nth(0).unwrap();
    let n: u32 = age_digit.parse().unwrap();
    n
}

#[cfg(test)]
mod tests {
    use super::get_age;
    #[test]
    fn basic_tests() {
        assert_eq!(get_age("2 years old"), 2);
        assert_eq!(get_age("4 years old"), 4);
        assert_eq!(get_age("5 years old"), 5);
        assert_eq!(get_age("7 years old"), 7);
    }
}