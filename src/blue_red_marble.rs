fn guess_blue(blue_start: u32, red_start: u32, blue_pulled: u32, red_pulled: u32) -> f32 {
    let b = blue_start - blue_pulled;
    let r = red_start - red_pulled;
    b as f32 / (b + r) as f32
}

#[cfg(test)]
mod tests {
    use super::guess_blue;

    #[test]
    fn basic_tests() {
      assert_eq!(guess_blue(5, 5, 2, 3), 0.6);
      assert_eq!(guess_blue(5, 7, 4, 3), 0.2);
      assert_eq!(guess_blue(12, 18, 4, 6), 0.4);
    }    
}
