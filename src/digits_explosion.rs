fn explode(orig: u64) -> u64 {
    let mut result = String::new();
    for d in orig.to_string().chars() {
        let n = d.to_digit(10).unwrap();
        result += &d.to_string().repeat(n as usize);
    }
    let ans: u64 = result.parse().unwrap();
    ans
}

#[cfg(test)]
mod tests {
    use super::explode;

    #[test]
    fn test0() {
        assert_eq!(333122, explode(312))
    }

    #[test]
    fn test1() {
        assert_eq!(12222666666999999999, explode(102269));
    }
}
