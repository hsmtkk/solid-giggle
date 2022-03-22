fn well_of_ideas(ss:&[&str]) -> String {
    let goods = ss.iter().filter(|s| **s == "good").count();
    let s = match goods {
        0 => "Fail!",
        1 | 2 => "Publish!",
        _ => "I smell a series!",
    };
    s.to_string()
}

#[cfg(test)]
mod tests {
    use super::well_of_ideas;

    #[test]
    fn test0(){
        let want = "I smell a series!";
        let ss = vec!["good", "bad", "good", "good", "bad", "good", "bad", "bad", "good", "bad", "bad"];
        let got = well_of_ideas(&ss);
        assert_eq!(want, got);
    }

    #[test]
    fn test1(){
        let want = "Publish!";
        let ss = vec!["bad", "bad", "bad", "bad", "good", "good", "bad", "bad", "bad"];
        let got = well_of_ideas(&ss);
        assert_eq!(want, got);
    }

    #[test]
    fn test2(){
        let want = "Fail!";
        let ss = vec!["bad", "bad"];
        let got = well_of_ideas(&ss);
        assert_eq!(want, got);
    }
}
