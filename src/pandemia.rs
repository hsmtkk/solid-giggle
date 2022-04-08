fn infected(s: &str) -> f64 {
    let splitted: Vec<String> = split_by_ocean(s);
    let infected: Vec<String> = splitted.iter().map(infect).collect();
    let joined = infected.join("");
    let all = joined.chars().filter(|ch| *ch == '0' || *ch == '1').count();
    let count = joined.chars().filter(|ch| *ch == '1').count();
    if all == 0 {
        return 0.0;
    }
    count as f64 * 100.0 / all as f64
}

fn split_by_ocean(s:&str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut buf = String::new();
    for ch in s.chars(){
        if ch == 'X' {
            result.push(buf);
            result.push("X".to_string());
            buf = String::new();
        } else {
            buf.push(ch);
        }
    }
    if ! buf.is_empty(){
        result.push(buf);
    }
    result
}

fn infect(s:&String) -> String {
    if s.contains('1') {
        "1".repeat(s.len())
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::infected;
    use super::split_by_ocean;

    fn approx_equals(sol: f64, exp: f64) {
        assert!((sol-exp).abs() <= 1e-9, "Got {}, expected: {} within a margin of 1e-9", sol, exp);
    }

    #[test]
    fn test_fixed() {
        let tests = vec![("01000000X000X011X0X",73.33333333333333),
                        ("01X000X010X011XX", 72.72727272727273),
                        ("XXXXX", 0.),
                        ("00000000X00X0000", 0.),
                        ("0000000010", 100.),
                        ("000001XXXX0010X1X00010", 100.),
                        ("X00X000000X10X0100",42.857142857142854)];
        tests.into_iter().for_each(|(world, exp)| { approx_equals(infected(world), exp); })
    }

    #[test]
    fn test_split_by_ocean(){
        let input = "01000000X000X011X0X";
        let got = split_by_ocean(input);
        let want = vec!["01000000","X","000","X","011","X","0","X"];
        assert_eq!(want, got);
    }
}