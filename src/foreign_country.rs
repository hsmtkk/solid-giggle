fn days_represented(trips: &[(u32, u32)]) -> u32 {
    let f = |d| {
        if tripping(d, trips) {
            1
        } else {
            0
        }
    };
    (1..366).map(f).sum()
}

fn tripping(day: u32, trips: &[(u32, u32)]) -> bool {
    for (begin, end) in trips {
        if *begin <= day && day <= *end {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::days_represented;

    #[test]
    fn sample_tests() {
        do_test(&[(10, 15), (25, 35)], 17);
        do_test(&[(2, 8), (220, 229), (10, 16)], 24);
        do_test(&[(2, 8), (6, 16), (17, 26)], 25);
        do_test(&[(1, 365)], 365);
    }

    fn do_test(trips: &[(u32, u32)], days: u32) {
        let user_result = days_represented(trips);
        assert!(
            user_result == days,
            "Test failed!\ntrips: {:?}\nGot: {}\nExpected: {}",
            trips,
            user_result,
            days
        );
    }
}
