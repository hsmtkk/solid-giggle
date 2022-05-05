fn consecutive(xs: &[i16]) -> i16 {
    if xs.len() <= 1 {
        return 0;
    }
    let min = *xs.iter().min().unwrap();
    let max = *xs.iter().max().unwrap();
    let mut count = 0;
    for i in min..max {
        if !xs.contains(&i) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(consecutive(&[4, 8, 6]), 2);
        assert_eq!(consecutive(&[1, 2, 3, 4]), 0);
        assert_eq!(consecutive(&[]), 0);
        assert_eq!(consecutive(&[1]), 0);
    }
}
