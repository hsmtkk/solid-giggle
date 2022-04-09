fn pyramid(balls: u16) -> u16 {
    let mut n = 1;
    loop {
        let m = n * (n+1) / 2;
        if m == balls {
            return n;
        } else if m > balls {
            return n - 1;
        }
        n += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(pyramid(1), 1);
        assert_eq!(pyramid(4), 2);
        assert_eq!(pyramid(20), 5);
        assert_eq!(pyramid(100), 13);
        assert_eq!(pyramid(2211), 66);
        assert_eq!(pyramid(9999), 140);
    }
}