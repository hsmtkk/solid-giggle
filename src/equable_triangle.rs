fn is_equable_triangle(a: u32, b: u32, c: u32) -> bool {
    area(a, b, c) == perimeter(a, b, c)
}

fn area(a: u32, b: u32, c: u32) -> f32 {
    let a = a as f32;
    let b = b as f32;
    let c = c as f32;
    let s = 0.5 * (a + b + c);
    (s * (s - a) * (s - b) * (s - c)).sqrt()
}

fn perimeter(a: u32, b: u32, c: u32) -> f32 {
    (a + b + c) as f32
}

#[cfg(test)]
mod tests {
    use super::is_equable_triangle;

    #[test]
    fn test0() {
        assert!(is_equable_triangle(5, 12, 13));
    }

    #[test]
    fn test1() {
        assert!(!is_equable_triangle(2, 3, 4));
    }
}
