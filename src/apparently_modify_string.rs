const APPARENTLY: &str = "apparently";

fn apparently(string: &str) -> String {
    let mut results = Vec::new();
    let words: Vec<&str> = string.split_whitespace().collect();
    for (index, &word) in words.iter().enumerate(){
        results.push(word);
        if word == "and" || word == "but" {
            if index == words.len()-1 {
                results.push(APPARENTLY);
            } else if words[index+1] != APPARENTLY {
                results.push(APPARENTLY);
            }
        }
    }
    results.join(" ")
}

fn test_exp(a: &str, exp: &str) {
    assert_eq!(apparently(a), exp.to_string());
}

#[test]
fn test_apparently() {
    test_exp("It was great and I have never been on live television before but sometimes I dont watch this.", "It was great and apparently I have never been on live television before but apparently sometimes I dont watch this.");
    test_exp("and", "and apparently");
    test_exp("apparently", "apparently");
    test_exp("but apparently", "but apparently");
}