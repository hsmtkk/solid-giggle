fn mul35(n: u32) -> u32 {
    (1..n).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_mul35() {
        assert_eq!(23, super::mul35(10));
    }
}
