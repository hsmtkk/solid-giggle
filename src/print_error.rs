fn printer_error(s: &str) -> String {
    let all = s.len();
    let error = s.chars().filter(is_error).count();
    format!("{}/{}", error, all)
}

fn is_error(ch:&char) -> bool {
    if 'a'<=*ch && *ch <='m' {
        false
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_pass_all_the_tests_provided() {
        assert_eq!(&printer_error("aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"), "3/56");
        assert_eq!(&printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"), "6/60");
        assert_eq!(&printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyzuuuuu"), "11/65");
    }
}
