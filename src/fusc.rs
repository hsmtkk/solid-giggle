fn fusc(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else if n % 2 == 0 {
        fusc(n / 2)
    } else {
        let k = (n - 1) / 2;
        fusc(k) + fusc(k + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::fusc;

    #[test]
    fn test_fusc() {
        assert_eq!(21, fusc(85));
    }
}
