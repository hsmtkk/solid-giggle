fn find_screen_height(width: u64, ratio: &str) -> String {
    let elems: Vec<&str>  = ratio.split(":").collect();
    let ratio_width:u64 = elems[0].parse().unwrap();
    let ratio_height:u64 = elems[1].parse().unwrap();
    let height = width * ratio_height / ratio_width;
    format!("{}x{}", width, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // 4:3, width = 768
        assert_eq!(find_screen_height(1024,"4:3"), "1024x768");

        // 16:9, width = 720
        assert_eq!(find_screen_height(1280,"16:9"), "1280x720");

        // 32:9, width = 1080
        assert_eq!(find_screen_height(3840,"32:9"), "3840x1080");
    }
}
