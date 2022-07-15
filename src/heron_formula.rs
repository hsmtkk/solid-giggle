fn heron(a:u32, b:u32, c:u32) -> f32 {
    let s: f32 = ((a  + b +c) as f32) / 2.0;
    (s * (s-a as f32) * (s-b as f32) * (s-c as f32)).sqrt()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_heron(){
        assert_eq!(6.0, super::heron(3,4,5));
    }
}