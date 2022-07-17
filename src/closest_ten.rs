fn closest_ten(n:u32) -> u32{
    ((n + 5) / 10) * 10
}

#[cfg(test)]
mod tests {
    use super::closest_ten;

    #[test]
    fn test_closest_ten(){
        assert_eq!(50, closest_ten(54));
        assert_eq!(60, closest_ten(55));
    }
}