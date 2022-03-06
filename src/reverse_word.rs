fn reverse_word(s:&str) -> String {
    let words: Vec<&str> = s.split(" ").collect();
    let reversed_words: Vec<String> = words.iter().map(reverse).collect();
    reversed_words.join(" ").to_string()
}

fn reverse(s:&&str) -> String {
    let mut result = String::new();
    for ch in s.chars(){
        result = ch.to_string() + &result;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::reverse_word;

    #[test]
    fn test_reverse_words() {
        assert_eq!("ehT kciuq nworb xof spmuj revo eht yzal .god", reverse_word("The quick brown fox jumps over the lazy dog."));
        assert_eq!("elbuod  decaps  sdrow", reverse_word("double  spaced  words"));
    }
}