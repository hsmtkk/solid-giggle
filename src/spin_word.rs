fn spin_words(words: &str) -> String {
    let reversed:Vec<String> = words.split(" ").map(reverse_word).collect();
    reversed.join(" ")
}

fn reverse_word(w:&str) -> String {
    if w.len()>4 {
        w.chars().rev().collect()
    } else {
        w.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(spin_words("Welcome"), "emocleW");
        assert_eq!(spin_words("Hey fellow warriors"), "Hey wollef sroirraw");
        assert_eq!(spin_words("This is a test"), "This is a test");
        assert_eq!(spin_words("This is another test"), "This is rehtona test");
        assert_eq!(spin_words("You are almost to the last test"), "You are tsomla to the last test");
        assert_eq!(spin_words("Just kidding there is still one more"), "Just gniddik ereht is llits one more");
        assert_eq!(spin_words("Seriously this is the last one"), "ylsuoireS this is the last one");
    }
}
