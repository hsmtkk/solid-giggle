fn is_in_middle(seq: &str) -> bool {
    println!("{}", seq);
    match seq.find("abc") {
        Some(index) => {
            let left = index;
            let right = seq.len() - left - 3;
            return diff(left, right) <= 1;
        }
        None => {
            return false;
        }
    }
}

fn diff(x: usize, y: usize) -> usize {
    if x >= y {
        x - y
    } else {
        y - x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(is_in_middle("AAabcBB"), true);
        assert_eq!(is_in_middle("AabcBB"), true);
        assert_eq!(is_in_middle("AabcBBB"), false);
    }
}
