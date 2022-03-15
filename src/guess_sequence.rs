fn sequence(x: u8) -> Vec<u8> {
    let mut ns: Vec<u8> = (1..x+1).collect();
    ns.sort_by_key(|x| x.to_string());
    ns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            sequence(16),
            [1, 10, 11, 12, 13, 14, 15, 16, 2, 3, 4, 5, 6, 7, 8, 9],
            "sequence(16)",
        );

        assert_eq!(
            sequence(9),
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            "sequence(9)",
        );
    }
}