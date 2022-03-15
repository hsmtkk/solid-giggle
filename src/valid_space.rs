fn valid_spacing(s: &str) -> bool {
    if s.starts_with(' '){
        return false;
    }
    if s.ends_with(' '){
        return false;
    }
    if continuous_space(s){
        return false;
    }
    true
}

fn continuous_space(s:&str) -> bool {
    println!("continuous? {}", s);
    let mut once = false;
    for ch in s.chars(){
        if once && ch == ' ' {
            println!("second space");
            return true;
        } else if ch == ' ' {
            println!("first space");
            once = true;
        } else {
            println!("non space {}", ch);
            once = false;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(valid_spacing("Hello world"), true, "Testing 'Hello world'");
        assert_eq!(valid_spacing(" Hello world"), false, "Testing ' Hello world'");
        assert_eq!(valid_spacing("Hello  world "), false, "Testing 'Hello  world '");
        assert_eq!(valid_spacing("Hello"), true, "Testing 'Hello'");
        assert_eq!(valid_spacing("Helloworld"), true, "Testing 'Helloworld'");
    }

    #[test]
    fn test0(){
        assert_eq!(valid_spacing("N cp  xZy"), false);
    }
}


