fn rot13(message: &str) -> String {
    message.chars().map(ro13).collect()
}

fn ro13(ch: char) -> char {
    let mut added: u8 = ch as u8 + 13_u8;
    if 'a' <= ch && ch <= 'z' {
        if added > 'z' as u8{
            added -= 26;
        }
    } else if 'A' <= ch && ch <= 'Z' {
        if added > 'Z' as u8{
            added -= 26;
        }
    } else {
        return ch;
    }
    return added as char;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(rot13("test"), "grfg");
        assert_eq!(rot13("Test"), "Grfg");
    }

    #[test]
    fn test_special(){
        assert_eq!(rot13("test-test"), "grfg-grfg");
    }
}