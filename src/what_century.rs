fn what_century(year: &str) -> String {
    let year: u32 = year.parse().expect("parse as u32");
    let century = if year % 100 == 0 {
        year / 100
    } else {
        year / 100 + 1
    };
    format!("{}{}", century, suffix(century))
}

fn suffix(century: u32) -> String {
    if century <= 20 {
        "th".to_string()
    } else {
        match century % 10 {
            1 => "st".to_string(),
            2 => "nd".to_string(),
            3 => "rd".to_string(),
            _ => "th".to_string(),
        }
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(what_century("1999"), "20th");
        assert_eq!(what_century("2011"), "21st");
        assert_eq!(what_century("2154"), "22nd");
        assert_eq!(what_century("2259"), "23rd");
        assert_eq!(what_century("1234"), "13th");
        assert_eq!(what_century("1023"), "11th");
        assert_eq!(what_century("2000"), "20th");
        assert_eq!(what_century("3210"), "33rd");
    }
}
