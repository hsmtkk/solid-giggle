fn find_difference(a: &[i32; 3], b: &[i32; 3]) -> i32 {
    let a3 = cube(a);
    let b3 = cube(b);
    diff(a3, b3)
}

fn cube(a:&[i32;3]) -> i32 {
    a[0] * a[1] * a[2]
}

fn diff(a:i32, b:i32) -> i32 {
    if a >= b {
        a - b
    } else {
        b - a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(find_difference(&[3, 2, 5], &[1, 4, 4]), 14);
        assert_eq!(find_difference(&[9, 7, 2], &[5, 2, 2]), 106);
        assert_eq!(find_difference(&[11, 2, 5], &[1, 10, 8]), 30);
        assert_eq!(find_difference(&[4, 4, 7], &[3, 9, 3]), 31);
        assert_eq!(find_difference(&[15, 20, 25], &[10, 30, 25]), 0);
    }
}
