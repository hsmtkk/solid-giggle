fn in_array(xs: &[&str], ys: &[&str]) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    for x in xs {
        if contain(x, ys) {
            words.push(x.to_string());
        }
    }
    words.sort();
    words.dedup();
    words
}

fn contain(x: &&str, ys: &[&str]) -> bool {
    for y in ys {
        if y.contains(*x) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::in_array;

    #[test]
    fn test0() {
        let got = in_array(
            &vec!["live", "arp", "strong"],
            &vec!["lively", "alive", "harp", "sharp", "armstrong"],
        );
        let want = vec!["arp", "live", "strong"];
        assert_eq!(want, got);
    }

    #[test]
    fn test1() {
        let got = in_array(&vec!["cod", "code", "wars", "ewar", "ar"], &vec![]);
        let want: Vec<String> = vec![];
        assert_eq!(want, got);
    }
}
