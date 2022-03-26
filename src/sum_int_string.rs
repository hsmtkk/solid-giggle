fn sum_int_string(s:&str) -> u32 {
    parse(s).iter().sum()
}

fn parse(s:&str) -> Vec<u32> {
    let mut results = Vec::new();
    let mut rest: String = s.to_string();
    let reg = regex::Regex::new(r"\d+").unwrap();
    while let Some(matched) = reg.find(&rest) {
        let begin = matched.start();
        let end = matched.end();
        let n: u32 = rest[begin..end].parse().unwrap();
        results.push(n);
        rest = rest[end..].to_string();
    }
    results
}

#[cfg(test)]
mod tests {
    use super::sum_int_string;

    #[test]
    fn test_sum_int_string(){
        let input = "The30quick20brown10f0x1203jumps914ov3r1349the102l4zy dog";
        let want = 3635;
        let got = sum_int_string(input);
        assert_eq!(want, got);
    }
}