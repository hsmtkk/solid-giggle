fn nb_year(p0: i32, percent: f32, aug: i32, p: i32) -> u32 {
    let mut current = p0;
    let mut year = 0;
    loop {
        year += 1;
        current += (current as f32 * percent * 0.01) as i32 + aug;
        if current >= p {
            break;
        }
    }
    year
}

#[cfg(test)]
mod tests {
    use super::nb_year;

    #[test]
    fn test0() {
        assert_eq!(15, nb_year(1500, 5.0, 100, 5000));
    }

    #[test]
    fn test1() {
        assert_eq!(10, nb_year(1500000, 2.5, 10000, 2000000));
    }

    #[test]
    fn test2() {
        assert_eq!(94, nb_year(1500000, 0.25, 1000, 2000000));
    }

    #[test]
    fn test3() {
        assert_eq!(151, nb_year(1500000, 0.25, -1000, 2000000));
    }
}
