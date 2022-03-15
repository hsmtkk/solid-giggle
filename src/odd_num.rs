fn odd_count(n: u32) -> u32 {
    if n % 2 == 0 {
        n / 2
    } else {
        (n - 1) / 2
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test15() {
        assert_eq!(7, super::odd_count(15));
    }

    #[test]
    fn test15023() {
        assert_eq!(7511, super::odd_count(15023));
    }
}
