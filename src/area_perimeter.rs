fn calculate(length: u32, width: u32) -> u32 {
    if length == width {
        length * width
    } else {
        2 * (length + width)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_perimeter() {
        let want = 32;
        let got = super::calculate(6, 10);
        assert_eq!(want, got);
    }

    #[test]
    fn test_area() {
        let want = 9;
        let got = super::calculate(3, 3);
        assert_eq!(want, got);
    }
}
