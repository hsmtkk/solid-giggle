fn amount_of_pages(summary: u32) -> u32 {
    let mut s: u32 = 0 ;
    let mut i = 1;
    loop {
        let d = i.to_string();
        s += d.len() as u32;
        if s >= summary {
            break;
        }
        i+=1;
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(amount_of_pages(5), 5);
        assert_eq!(amount_of_pages(25), 17);
        assert_eq!(amount_of_pages(1095), 401);
        assert_eq!(amount_of_pages(185), 97);
        assert_eq!(amount_of_pages(660), 256);
    }
}
