fn int32_to_ip(int: u32) -> String {
    let digits: Vec<String> = int32_binary(int).iter().map(binary_int).collect();
    digits.join(".")
}

fn binary_int(s:&String) -> String {
    let i = i32::from_str_radix(s, 2).expect("from str radix");
    i.to_string()
}

fn int32_binary(int:u32) -> Vec<String> {
    let s = format!("{:032b}", int);
    vec![s[0..8].to_string(), s[8..16].to_string(), s[16..24].to_string(), s[24..32].to_string()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(int32_to_ip(2154959208), "128.114.17.104");
        assert_eq!(int32_to_ip(2149583361), "128.32.10.1");
        assert_eq!(int32_to_ip(0), "0.0.0.0");
    }

    #[test]
    fn test_int32_binary(){
        let input = 2149583361;
        let got = super::int32_binary(input);
        let want = vec!["10000000","00100000","00001010", "00000001"];
        assert_eq!(want, got);
    }
}
