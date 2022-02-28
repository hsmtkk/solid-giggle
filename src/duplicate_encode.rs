use std::collections::HashMap;

fn duplicate_encode(word:&str)->String {
    let word = word.to_lowercase();
    let mut char_count: HashMap<char, u32> = HashMap::new();
    for ch in word.chars(){
        match char_count.get(&ch){
            Some(count) => {
                char_count.insert(ch, count + 1);
            },
            None => {
                char_count.insert(ch, 1);
            },
        }
    }
    let mut result = String::new();
    for ch in word.chars(){
        let count = char_count.get(&ch).unwrap();
        result += if *count == 1{
            "("
        } else {
            ")"
        };
    }
    result
}

#[cfg(test)]
mod tests {
    use super::duplicate_encode;
    #[test]
    fn run_tests() {
        assert_eq!(duplicate_encode("din"),"(((");
        assert_eq!(duplicate_encode("recede"),"()()()");
        assert_eq!(duplicate_encode("Success"),")())())","should ignore case");
        assert_eq!(duplicate_encode("(( @"),"))((");
    }
}
