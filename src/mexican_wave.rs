fn wave(word:&str) -> Vec<String> {
    let mut results = Vec::new();
    for i in 0..word.len(){
        let modified = modify(word, i);
        if word == modified {
            continue;
        }
        results.push(modified);
    }
    results
}

fn modify(word:&str, index:usize) -> String {
    if index == 0 {
        word.chars().nth(0).unwrap().to_uppercase().to_string() + &word[index+1..]
    } else if index == word.len() - 1 {
        let mut s: String = word[..index].to_string();
        s + &word.chars().nth(index).unwrap().to_uppercase().to_string()
    } else {
        let mut s: String = word[..index].to_string();
        s += &word.chars().nth(index).unwrap().to_uppercase().to_string();
        s + &word[index+1..]
    }
}

#[cfg(test)]
mod tests {
    use super::wave;

    struct TestCase {
        input: String,
        want: Vec<String>,
    }

    impl TestCase {
        fn new(input:&str, want:Vec<&str>) -> Self {
            TestCase{input:input.to_string(), want:want.iter().map(|s| s.to_string()).collect()}
        }
    }

    #[test]
    fn test_wave(){
        let test_cases = vec![
            TestCase::new(" x yz",     vec![" X yz", " x Yz", " x yZ"]),
            TestCase::new("abc", vec!["Abc", "aBc", "abC"]),
        ];
        for test_case in test_cases {
            let got = wave(&test_case.input);
            assert_eq!(test_case.want, got);
        }
    }
}

/*
		,
		:       []string{},
		" ab  c":    []string{" Ab  c", " aB  c", " ab  C"},
		"":          []string{},
		"z":         []string{"Z"},
		"a a a a a": []string{"A a a a a", "a A a a a", "a a A a a", "a a a A a", "a a a a A"},
		"aaaaa":     []string{"Aaaaa", "aAaaa", "aaAaa", "aaaAa", "aaaaA"},
		"                                                           ": []string{},
		" a  b     c  defghijk l  m no pqrs tuvwx yz     ":            []string{" A  b     c  defghijk l  m no pqrs tuvwx yz     ", " a  B     c  defghijk l  m no pqrs tuvwx yz     ", " a  b     C  defghijk l  m no pqrs tuvwx yz     ", " a  b     c  Defghijk l  m no pqrs tuvwx yz     ", " a  b     c  dEfghijk l  m no pqrs tuvwx yz     ", " a  b     c  deFghijk l  m no pqrs tuvwx yz     ", " a  b     c  defGhijk l  m no pqrs tuvwx yz     ", " a  b     c  defgHijk l  m no pqrs tuvwx yz     ", " a  b     c  defghIjk l  m no pqrs tuvwx yz     ", " a  b     c  defghiJk l  m no pqrs tuvwx yz     ", " a  b     c  defghijK l  m no pqrs tuvwx yz     ", " a  b     c  defghijk L  m no pqrs tuvwx yz     ", " a  b     c  defghijk l  M no pqrs tuvwx yz     ", " a  b     c  defghijk l  m No pqrs tuvwx yz     ", " a  b     c  defghijk l  m nO pqrs tuvwx yz     ", " a  b     c  defghijk l  m no Pqrs tuvwx yz     ", " a  b     c  defghijk l  m no pQrs tuvwx yz     ", " a  b     c  defghijk l  m no pqRs tuvwx yz     ", " a  b     c  defghijk l  m no pqrS tuvwx yz     ", " a  b     c  defghijk l  m no pqrs Tuvwx yz     ", " a  b     c  defghijk l  m no pqrs tUvwx yz     ", " a  b     c  defghijk l  m no pqrs tuVwx yz     ", " a  b     c  defghijk l  m no pqrs tuvWx yz     ", " a  b     c  defghijk l  m no pqrs tuvwX yz     ", " a  b     c  defghijk l  m no pqrs tuvwx Yz     ", " a  b     c  defghijk l  m no pqrs tuvwx yZ     "},

*/