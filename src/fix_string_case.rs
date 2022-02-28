fn solve(s: &str) -> String {
    let lowers = count_lower(s);
    let uppers = count_upper(s);
    if lowers >= uppers {
        s.to_lowercase()
    } else {
        s.to_uppercase()
    }
}

fn count_lower(s: &str) -> u32 {
    s.chars().fold(0, cnt_lower)
}

fn cnt_lower(acc:u32, ch:char) -> u32 {
    if 'a' <= ch && ch <= 'z' {
        acc + 1
    } else {
        acc
    }
}

fn count_upper(s: &str) -> u32 {
    s.chars().fold(0, cnt_upper)
}

fn cnt_upper(acc:u32, ch:char) -> u32 {
    if 'A' <= ch && ch <= 'Z' {
        acc + 1
    } else {
        acc
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solve("code"), "code");
        assert_eq!(solve("CODe"), "CODE");
        assert_eq!(solve("COde"), "code");
        assert_eq!(solve("Code"), "code");
    }
}
