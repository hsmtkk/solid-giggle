use std::collections::HashMap;

fn duplicate_count(s: &str) -> usize {
    let mut char_counter: HashMap<char, u32> = HashMap::new();
    for ch in s.to_lowercase().chars() {
        match char_counter.get(&ch) {
            Some(count) => {
                char_counter.insert(ch, count + 1);
            }
            None => {
                char_counter.insert(ch, 1);
            }
        }
    }
    char_counter.retain(|_, v| *v > 1);
    char_counter.len()
}

#[cfg(test)]
mod tests {
    use super::duplicate_count;

    #[test]
    fn test0() {
        assert_eq!(0, duplicate_count("abcde"));
    }

    #[test]
    fn test1() {
        assert_eq!(1, duplicate_count("abcdea"));
    }

    #[test]
    fn test2() {
        assert_eq!(3, duplicate_count("abcdeaB11"));
    }

    #[test]
    fn test3() {
        assert_eq!(1, duplicate_count("indivisibility"));
    }
}
