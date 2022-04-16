fn print(n: i32) -> Option<String> {
    if n <= 0 || n % 2 == 0 {
        return None;
    }
    let m = (n - 1) / 2;
    let mut result = String::new();
    result += &upper(m as usize);
    result += &"*".repeat(n as usize);
    result += "\n";
    result += &lower(m as usize);
    Some(result)
}

fn upper(m: usize) -> String {
    let mut result = String::new();
    for i in 0..m {
        result += &" ".repeat(m - i);
        result += &"*".repeat(i * 2 + 1);
        result += "\n";
    }
    result
}

fn lower(m: usize) -> String {
    let mut result = String::new();
    for i in 0..m {
        result += &" ".repeat(i + 1);
        result += &"*".repeat((m - i) * 2 - 1);
        result += "\n";
    }
    result
}

#[cfg(test)]
mod tests {
    use super::print;

    #[test]
    fn basic_test() {
        assert_eq!(print(3), Some(" *\n***\n *\n".to_string()));
        assert_eq!(print(5), Some("  *\n ***\n*****\n ***\n  *\n".to_string()));
        assert_eq!(print(-3), None);
        assert_eq!(print(2), None);
        assert_eq!(print(0), None);
        assert_eq!(print(1), Some("*\n".to_string()));
    }
}
