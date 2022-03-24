const GRAVITY: f32 = 9.81;

fn max_ball(v0: f32) -> u32 {
    let v0 = v0 * 1000.0 / 3600.0;
    let mut max = 0.0;
    let mut maxi = 0;
    let mut i = 0;
    loop {
        let t = 0.1 * i as f32;
        let h = calc_height(v0, t);
        if h < 0.0 {
            break;
        }
        if h > max {
            max = h;
            maxi = i;
        }
        i += 1;
    }
    maxi
}

fn calc_height(v0: f32, t: f32) -> f32 {
    v0 * t - 0.5 * GRAVITY * t * t
}

#[cfg(test)]
mod tests {
    use super::max_ball;

    #[test]
    fn test0() {
        assert_eq!(10, max_ball(37.0));
    }

    #[test]
    fn test1() {
        assert_eq!(13, max_ball(45.0));
    }

    #[test]
    fn test2() {
        assert_eq!(28, max_ball(99.0));
    }

    #[test]
    fn test3() {
        assert_eq!(24, max_ball(85.0));
    }

    #[test]
    fn test4() {
        assert_eq!(39, max_ball(136.0));
    }

    #[test]
    fn test5() {
        assert_eq!(4, max_ball(15.0));
    }
}
