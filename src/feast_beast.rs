fn feast(beast: &str, dish: &str) -> bool {
    let beast_head = beast.chars().next().unwrap();
    let beast_tail = beast.chars().rev().next().unwrap();
    let dish_head = dish.chars().next().unwrap();
    let dish_tail = dish.chars().rev().next().unwrap();
    beast_head == dish_head && beast_tail == dish_tail
}

#[cfg(test)]
mod tests {
    use super::feast;

    // Rust test example:
    #[test]
    fn sample_test_cases() {
        assert_eq!(feast("great blue heron", "garlic naan"), true);
        assert_eq!(feast("chickadee", "chocolate cake"), true);
        assert_eq!(feast("brown bear", "bear claw"), false);
    }
}
