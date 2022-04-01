fn ips_between(start: &str, end: &str) -> u32 {
    let from = ip_to_int(start);
    let to = ip_to_int(end);
    to - from
}

fn ip_to_int(ip: &str) -> u32 {
    let elems: Vec<&str> = ip.split('.').collect();
    let ns: Vec<u32> = elems.iter().map(|s| s.parse().unwrap()).collect();
    (ns[0] << 24) + (ns[1] << 16) + (ns[2] << 8) + ns[3]
}

#[cfg(test)]
mod tests {
    use super::ips_between;

    #[test]
    fn test0() {
        assert_eq!(50, ips_between("10.0.0.0", "10.0.0.50"));
    }

    #[test]
    fn test1() {
        assert_eq!(256, ips_between("10.0.0.0", "10.0.1.0"));
    }

    #[test]
    fn test2() {
        assert_eq!(246, ips_between("20.0.0.10", "20.0.1.0"));
    }
}
