fn solve(s: &str, k: usize) -> String {
    let mut count = k;
    let mut chs: Vec<char> = s.chars().collect();
    for alpha in "abcdefghijklmnopqrstuvwxyz".chars(){
        count -= replace(&mut chs, alpha, count);
        if count <= 0 {
            break;
        }
    }
    let t: String = chs.iter().collect();
    t.replace(".", "")
}

fn replace(chars: &mut Vec<char>, alpha: char, count: usize) -> usize {
    let mut replaced = 0;
    for i in 0..chars.len(){
        if chars[i] == alpha {
            chars[i] = '.';
            replaced += 1;
        }
        if replaced >= count {
            break;
        }
    }
    replaced
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(solve("abracadabra", 1), "bracadabra".to_string());
        assert_eq!(solve("abracadabra", 2), "brcadabra".to_string());
        assert_eq!(solve("abracadabra", 6), "rcdbr".to_string());
        assert_eq!(solve("abracadabra", 8), "rdr".to_string());
        assert_eq!(solve("abracadabra", 50), "".to_string());
    }
}
