fn evaporator(init_content: u32, evap_per_day: u32, threshold_ratio: u32) -> u32 {
    let mut content: f32 = init_content as f32;
    let threshold_content: f32 = init_content as f32 * threshold_ratio as f32 * 0.01;
    let mut days = 0;
    loop {
        days += 1;
        content *= 1.0 - 0.01 * evap_per_day as f32;
        if content < threshold_content {
            break;
        }
    }
    days
}

#[cfg(test)]
mod tests {
    use super::evaporator;

    #[test]
    fn test0() {
        assert_eq!(22, evaporator(10, 10, 10));
    }

    #[test]
    fn test1() {
        assert_eq!(29, evaporator(10, 10, 5));
    }

    #[test]
    fn test2() {
        assert_eq!(59, evaporator(100, 5, 5));
    }
}
