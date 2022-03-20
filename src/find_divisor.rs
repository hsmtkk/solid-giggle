fn divisors(integer: u32) -> Result<Vec<u32>, String> {
    let ns: Vec<u32> = (2..integer).filter(|x| integer % x == 0).collect();
    if ns.len() == 0 {
        return Err(format!("{} is prime", integer));
    }
    Ok(ns)
}

#[cfg(test)]
mod tests {
    use super::divisors;
    #[test]
    fn tests() {
        assert_eq!(divisors(15), Ok(vec![3, 5]));
        assert_eq!(divisors(12), Ok(vec![2, 3, 4, 6]));
        assert_eq!(divisors(13), Err("13 is prime".to_string()));
    }
}
