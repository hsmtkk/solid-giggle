fn largest_five_digit_number(num: &str) -> u32 {
    *sequences(num).iter().max().expect("get max")
}

fn sequences(digits: &str) -> Vec<u32> {
    let mut sqs = Vec::new();
    for i in 0..digits.len() - 5 + 1 {
        let ds = &digits[i..i + 5];
        let n: u32 = ds.parse().expect("parse as u32");
        sqs.push(n);
    }
    sqs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(largest_five_digit_number(&"1234567890"), 67890);
        assert_eq!(largest_five_digit_number(&"731674765"), 74765);
    }

    #[test]
    fn test_sequences() {
        let want = vec![12345, 23456, 34567, 45678, 56789, 67890];
        let got = super::sequences("1234567890");
        assert_eq!(want, got);
    }
}
