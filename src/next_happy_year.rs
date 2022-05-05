fn int_to_digits(n:u16) -> Vec<u16> {
    let mut digits = Vec::new();
    let mut m = n;
    loop {
        digits.push(m % 10);
        m /= 10;
        if m == 0 {
            break;
        }
    }
    digits.reverse();
    digits
}

fn next_happy_year(year: u16) -> u16 {
    let mut y = year;
    loop {
        y += 1;
        if is_distinct(&int_to_digits(y)) {
            return y;
        }
    }
}

fn is_distinct(digits:&[u16]) -> bool {
    let mut ds = digits.to_vec();
    ds.sort();
    ds.dedup();
    digits.len() == ds.len()
}

#[cfg(test)]
mod tests {
    use super::next_happy_year;

    #[test]
    fn test_int_to_digits(){
        assert_eq!(vec![1,2,3,4], super::int_to_digits(1234));
        assert_eq!(vec![7,8,9,0], super::int_to_digits(7890));
    }

    #[test]
    fn basic() {
        assert_eq!(next_happy_year(1001), 1023);
        assert_eq!(next_happy_year(1123), 1203);
        assert_eq!(next_happy_year(2001), 2013);
        assert_eq!(next_happy_year(2334), 2340);
        assert_eq!(next_happy_year(3331), 3401);
        assert_eq!(next_happy_year(1987), 2013);
        assert_eq!(next_happy_year(5555), 5601);
        assert_eq!(next_happy_year(7712), 7801);
        assert_eq!(next_happy_year(8088), 8091);
        assert_eq!(next_happy_year(8999), 9012);
    }
}