use num_bigint::BigUint;

fn nb_dig(n :u32, d:u32) -> u32 {
    let sqs: Vec<BigUint> = (0..n+1).into_iter().map(square).collect();
    let mut count = 0;
    for sq in sqs {
        let s = sq.to_string();
        for ch in s.chars() {
            if ch == char::from_digit(d, 10).unwrap() {
                count+=1
            }
        }
    }
    count
}

fn square(n:u32) -> BigUint {
    let bn = BigUint::from(n);
    &bn * &bn
}

#[cfg(test)]
mod tests {
    use super::nb_dig;

    #[test]
    fn test0(){
        let got = nb_dig(10, 1);
        assert_eq!(4, got);
    }

    #[test]
    fn test1(){
        let got = nb_dig(25, 1);
        assert_eq!(11, got);
    }

    #[test]
    fn test2(){
        assert_eq!(213, nb_dig(550, 5));
    }

    #[test]
    fn test3(){
        assert_eq!(4700, nb_dig(5750, 0));
    }
}
