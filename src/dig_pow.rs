fn dig_pow(n:u64, p:u32) -> Option<u64> {
    let dp = calc_dig_pow(n, p);
    let mut k = 0;
    loop {
        let product = n * k;
        if product == dp {
            return Some(k);
        } else if product > dp {
            return None;
        }
        k+=1;
    }
}

fn calc_dig_pow(n:u64, p:u32) -> u64 {
    let mut s: u64 = 0;
    let ds = n.to_string();
    for i in 0..ds.len(){
        let d: char = ds.chars().nth(i).unwrap();
        let m = d.to_digit(10).unwrap();
        s += m.pow(i as u32 + p) as u64;
    }
    s
}

#[cfg(test)]
mod tests {
    use super::dig_pow;

    #[test]
    fn test0(){
        assert_eq!(Some(1), dig_pow(89, 1))
    }

    #[test]
    fn test1(){
        assert_eq!(None, dig_pow(92, 1))
    }

    #[test]
    fn test2(){
        assert_eq!(Some(51), dig_pow(46288, 3))
    }
}