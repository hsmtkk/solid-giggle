fn movie(card: u32, ticket: u32, perc: f64) -> u32 {
    let mut n = 1;
    loop {
        let a = system_a(ticket, n);
        let b = system_b(card, ticket, perc, n);
        if a > b.ceil() as u32 {
            return n;
        }
        n += 1;
    }
}

fn system_a(ticket: u32, n: u32) -> u32 {
    n * ticket
}

fn system_b(card: u32, ticket: u32, perc: f64, n: u32) -> f64 {
    card as f64 + ticket as f64 * perc * (1.0 - perc.powi(n as i32)) / (1.0 - perc)
}

#[cfg(test)]
mod tests {
    use super::movie;

    #[test]
    fn test0() {
        assert_eq!(43, movie(500, 15, 0.9));
    }

    #[test]
    fn test1() {
        assert_eq!(2, movie(0, 10, 0.95));
    }
}
