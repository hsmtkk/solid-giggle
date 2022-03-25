fn is_triangle_number(n: u64) -> bool {
    match n {
        0 | 1 => {return true;},
        _ => {},
    }
    let mut k = 1;
    loop {
        let t = triangle_number(k);
        if t == n {
            return true;
        } else if t > n {
            return false;
        }
        k += 1;
    }
}

fn triangle_number(n:u64) -> u64 {
    n * (n + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(is_triangle_number(3), true);
        assert_eq!(is_triangle_number(5), false);
        assert_eq!(is_triangle_number(8), false);
        assert_eq!(is_triangle_number(10), true);
        assert_eq!(is_triangle_number(20), false);
    }

    #[test]
    fn test_zero_and_one() {
        assert_eq!(is_triangle_number(0), true);
        assert_eq!(is_triangle_number(1), true);
    }
}

