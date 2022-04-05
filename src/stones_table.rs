fn stones_to_remove(stones: &str) -> usize {
    let mut prev = stones.chars().nth(0).unwrap();
    let mut count = 0;
    for i in 1..stones.len() {
        let current = stones.chars().nth(i).unwrap();
        if current == prev {
            count += 1;
        } else {
            prev = current;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::stones_to_remove;

    #[test]
    fn sample_tests() {
        assert_eq!(stones_to_remove("RGBRGBRGGB"), 1);
        assert_eq!(stones_to_remove("RGGRGBBRGRR"), 3);
        assert_eq!(stones_to_remove("RRRRGGGGBBBB"), 9);
    }
}
