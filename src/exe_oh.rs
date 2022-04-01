fn xo(string: &'static str) -> bool {
    let x = string.chars().filter(|ch| *ch == 'x' || *ch == 'X').count();
    let o = string.chars().filter(|ch| *ch == 'o' || *ch == 'O').count();
    x == o
}

#[cfg(test)]
mod tests {
    use super::xo;

    #[test]
    fn returns_expected() {
        assert_eq!(xo("xo"), true);
        assert_eq!(xo("Xo"), true);
        assert_eq!(xo("xxOo"), true);
        assert_eq!(xo("xxxm"), false);
        assert_eq!(xo("Oo"), false);
        assert_eq!(xo("ooom"), false);
    }
}
