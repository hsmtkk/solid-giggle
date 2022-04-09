const PI_STR: &str = "31415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679";

fn square_pi(digits:u32) -> u32 {
    let mut sum = 0;
    for i in 0..digits {
        let d = PI_STR.chars().nth(i.try_into().unwrap()).unwrap();
        let n = d.to_digit(10).unwrap();
        sum += n * n;
    }
    (sum as f32).sqrt().ceil() as u32
}

#[cfg(test)]
mod tests {
    use super::square_pi;

    #[test]
    fn test0(){
        assert_eq!(8, square_pi(5));
    }

    #[test]
    fn test1(){
        assert_eq!(15, square_pi(10));
    }
}