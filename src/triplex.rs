fn triple_x(s: &str) -> bool {
    if let Some(triple_x_index) = s.find("xxx") {
        if let Some(single_x_index) = s.find("x") {
            if triple_x_index == single_x_index {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(triple_x("abraxxxas"), true);
        assert_eq!(triple_x("xoxotrololololololoxxx"), false);
        assert_eq!(triple_x("soft kitty, warm kitty, xxxxx"), true);
        assert_eq!(triple_x("softx kitty, warm kitty, xxxxx"), false);

        assert_eq!(triple_x("Xabraxxxas"), true);
        assert_eq!(triple_x("xoXotrololololololoxxx"), false);
        assert_eq!(triple_x("softXxxx kitty, warm kitty, xxxxx"), true);
        assert_eq!(triple_x("softxXxxx kitty, warm kitty, xxxxx"), false);
    }
}
