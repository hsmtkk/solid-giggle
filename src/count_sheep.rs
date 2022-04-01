fn count_sheep(num: u32) -> String {
    let mut ss = Vec::new();
    for i in 1..num + 1 {
        ss.push(format!("{} sheep...", i));
    }
    ss.join("")
}

#[cfg(test)]
mod tests {
    use super::count_sheep;

    #[test]
    fn test0() {
        assert_eq!("", count_sheep(0));
    }

    #[test]
    fn test1() {
        assert_eq!("1 sheep...", count_sheep(1));
    }

    #[test]
    fn test2() {
        assert_eq!("1 sheep...2 sheep...", count_sheep(2));
    }
}
