fn count_salutes(hallway: &str) -> usize {
    let mut meets = 0;
    for i in 0..hallway.len() {
        if hallway.chars().nth(i).unwrap() == '>' {
            meets += count_left(&hallway[i..]);
        }
    }
    meets * 2
}

fn count_left(hallway: &str) -> usize {
    hallway.chars().filter(|ch| *ch == '<').count()
}

#[cfg(test)]
mod tests {
    use super::count_salutes;

    #[test]
    fn basic() {
        assert_eq!(count_salutes("<---->---<---<-->"), 4);
        assert_eq!(count_salutes("------"), 0);
        assert_eq!(count_salutes(">>>>>>>>>>>>>>>>>>>>>----<->"), 42);
        assert_eq!(count_salutes("<<----<>---<"), 2);
        assert_eq!(count_salutes(">"), 0);
    }
}
