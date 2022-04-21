use std::iter::zip;

fn good_vs_evil(good: &str, evil: &str) -> String {
    let good_counts = parse_as_counts(good);
    let evil_counts = parse_as_counts(evil);
    let good_worth = vec![1, 2, 3,3, 4, 10];
    let evil_worth = vec![1,2,2,2,3,5,10];
    let good_points = zip(good_counts, good_worth).fold(0, |acc, x| acc + x.0 * x.1);
    let evil_points = zip(evil_counts, evil_worth).fold(0, |acc, x| acc + x.0 * x.1);
    if good_points > evil_points {
        "Battle Result: Good triumphs over Evil".to_string()
    } else if evil_points > good_points {
        "Battle Result: Evil eradicates all trace of Good".to_string()
    } else {
        "Battle Result: No victor on this battle field".to_string()
    }
}

fn parse_as_counts(s:&str) -> Vec<u32> {
    let ss: Vec<&str> = s.split_whitespace().collect();
    ss.into_iter().map(|s| s.parse::<u32>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::good_vs_evil;

    #[test]
    fn returns_expected() {
        assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0"), "Battle Result: Good triumphs over Evil");
        assert_eq!(good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 10"), "Battle Result: Evil eradicates all trace of Good");
        assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 10"), "Battle Result: No victor on this battle field");
    }
}