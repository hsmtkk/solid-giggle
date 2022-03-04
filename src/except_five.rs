fn dont_give_me_five(start: isize, end: isize) -> isize {
    (start..end+1).filter(except_five).collect::<Vec<isize>>().len() as isize
}

fn except_five(i:&isize) -> bool {
    let s: String = i.to_string();
    !s.contains("5")
}

// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[cfg(test)]
mod tests {
    use super::dont_give_me_five;

    #[test]
    fn returns_expected() {
        assert_eq!(dont_give_me_five(1, 9), 8);
        assert_eq!(dont_give_me_five(4, 17), 12);
    }
}
