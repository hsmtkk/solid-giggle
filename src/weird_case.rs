fn weird_case(line:&str) -> String {
    let words: Vec<&str> = line.split_whitespace().collect();
    let converted_words: Vec<String> = words.iter().map(convert).collect();
    converted_words.join(" ")
}

fn convert(word:&&str) -> String {
    word.chars().enumerate().map(convert2).collect()
}

fn convert2(pair:(usize, char)) -> char {
    if pair.0 % 2 == 0 {
        pair.1.to_ascii_uppercase()
    } else {
        pair.1.to_ascii_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::weird_case;

    #[test]
    fn test0(){
        assert_eq!("AbC DeF", weird_case("abc def"));
    }

    #[test]
    fn test1(){
        assert_eq!("AbC", weird_case("ABC"));
    }

    #[test]
    fn test2(){
        assert_eq!("ThIs Is A TeSt LoOkS LiKe YoU PaSsEd", weird_case("This is a test Looks like you passed"));
    }
}
