fn warn_the_sheep(queue: &[&str]) -> String {
    if queue[queue.len() - 1] == "wolf" {
        return "Pls go away and stop eating my sheep".to_string();
    }
    let index = queue.iter().position(|s| *s == "wolf").unwrap();
    let num = queue.len() - index - 1;
    format!(
        "Oi! Sheep number {}! You are about to be eaten by a wolf!",
        num
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            warn_the_sheep(&[
                "sheep", "sheep", "sheep", "sheep", "sheep", "wolf", "sheep", "sheep"
            ]),
            "Oi! Sheep number 2! You are about to be eaten by a wolf!",
        );
        assert_eq!(
            warn_the_sheep(&["sheep", "wolf", "sheep", "sheep", "sheep", "sheep", "sheep"]),
            "Oi! Sheep number 5! You are about to be eaten by a wolf!",
        );
        assert_eq!(
            warn_the_sheep(&["wolf", "sheep", "sheep", "sheep", "sheep", "sheep", "sheep"]),
            "Oi! Sheep number 6! You are about to be eaten by a wolf!",
        );
        assert_eq!(
            warn_the_sheep(&["sheep", "wolf", "sheep"]),
            "Oi! Sheep number 1! You are about to be eaten by a wolf!",
        );
        assert_eq!(
            warn_the_sheep(&["sheep", "sheep", "wolf"]),
            "Pls go away and stop eating my sheep",
        );
    }
}
