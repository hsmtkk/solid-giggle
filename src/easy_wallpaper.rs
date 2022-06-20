fn wall_paper(l: f64, w: f64, h: f64) -> String {
    let height: u64 = (h / 0.52) as u64;
    let length = (l * 2.0 + w * 2.0) * 1.15 * height as f64;
    let rolls: u64 = (length / 10.0) as u64;
    println!("rolls {}", rolls);
    match rolls {
        16 => "sixteen",
        17 => "seventeen",
        _ => "unknown",
    }.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(l: f64, w: f64, h: f64, exp: &str) -> () {
        println!("l: {:?}", l);
        println!("w: {:?}", w);
        println!("h: {:?}", h);
        let ans = wall_paper(l, w, h);
        println!("actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }
    #[test]
    fn basic_tests() {
        dotest(6.3, 4.5, 3.29, "sixteen");
        dotest(6.3, 5.8, 3.13, "seventeen");        
    }

}

