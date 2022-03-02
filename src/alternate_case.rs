fn to_alternate_case(orig:&str) -> String {
    orig.chars().map(alter).collect()
}

fn alter(ch:char) -> char {
    if ch.is_lowercase(){
        ch.to_uppercase().next().unwrap()
    } else if ch.is_uppercase(){
        ch.to_lowercase().next().unwrap()
    } else {
        ch
    }
}

#[cfg(test)]
mod tests {
    use super::to_alternate_case;

    #[test]
    fn test0(){
        let orig = "String.prototype.toAlternatingCase";
        let want = "sTRING.PROTOTYPE.TOaLTERNATINGcASE";
        let got = to_alternate_case(orig);
        assert_eq!(want,got);
    }

    #[test]
    fn test1(){
        let orig = "altERnaTIng cAsE";
        let want = "ALTerNAtiNG CaSe";
        let got = to_alternate_case(orig);
        assert_eq!(want,got);
    }
}
