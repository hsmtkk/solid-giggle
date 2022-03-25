fn accum(s: &str) -> String {
    let mut ss: Vec<String> = Vec::new();
    for i in 0..s.len() {
        let orig: char = s.chars().nth(i).unwrap();
        let mut s = orig.to_uppercase().to_string();
        for _k in 0..i {
            s += &orig.to_lowercase().to_string();
        }
        ss.push(s);
    }
    ss.join("-")
}

#[cfg(test)]
mod tests {
    use super::accum;

    #[test]
    fn test_accum() {
        let input_wants = vec![(
            "ZpglnRxqenU",
            "Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuuu",
        )];
        for (input, want) in input_wants {
            let got = accum(input);
            assert_eq!(want, got);
        }
    }
}
