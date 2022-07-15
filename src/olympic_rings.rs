fn olympic_ring(s: &str) -> String {
    let rings = s.chars().map(count_ring).sum::<u32>() / 2;
    match rings {
        0 | 1 => "Not even a medal!",
        2 => "Bronze!",
        3 => "Silver!",
        _ => "Gold!",
    }
    .to_string()
}

fn count_ring(ch: char) -> u32 {
    match ch {
        'a' => 1,
        'b' => 1,
        'd' => 1,
        'e' => 1,
        'g' => 1,
        'o' => 1,
        'p' => 1,
        'q' => 1,
        'A' => 1,
        'B' => 2,
        'D' => 1,
        'O' => 1,
        'P' => 1,
        'Q' => 1,
        'R' => 1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::olympic_ring;

    #[test]
    fn basic() {
        assert_eq!(olympic_ring("wHjMudLwtoPGocnJ"), "Bronze!", "test-0");
        assert_eq!(olympic_ring("eCEHWEPwwnvzMicyaRjk"), "Bronze!", "test-1");
        assert_eq!(olympic_ring("JKniLfLW"), "Not even a medal!", "test-2");
        assert_eq!(olympic_ring("EWlZlDFsEIBufsalqof"), "Silver!", "test-3");
        assert_eq!(olympic_ring("IMBAWejlGRTDWetPS"), "Gold!", "test-4");
    }
}
