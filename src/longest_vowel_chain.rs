fn solve(s: &str) -> usize {
	for i in (0..s.len()).rev() {
		if let Some(chain) = find_vowel_chain(s, i){
			return chain.len();
		}
	}
	0
}

fn find_vowel_chain(s:&str, length:usize) -> Option<String>{
	for i in 0..s.len()-length {
		let sub_str = &s[i..i+length];
		if is_vowel_chain(sub_str){
			return Some(sub_str.to_string());
		}
	}
	None
}

fn is_vowel_chain(s:&str) -> bool {
	for ch in s.chars(){
		if ch != 'a' && ch != 'i' && ch != 'u' && ch != 'e' && ch != 'o' {
			return false;
		}
	}
	true
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        let input_wants = maplit::hashmap!{
        "codewarriors" => 2,
        "suoidea"=> 3,
        "ultrarevolutionariees"=> 3,
        "strengthlessnesses"=> 1,
        "cuboideonavicuare"=> 2,
        "chrononhotonthuooaos"=> 5,
        "iiihoovaeaaaoougjyaw"=> 8,
        };
        for (input, want) in input_wants {
            let got = super::solve(input);
            assert_eq!(want, got);
        }
    }
}
