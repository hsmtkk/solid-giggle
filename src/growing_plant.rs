fn growing_plant(up_speed: u32, down_speed: u32, desired_height: u32) -> u32 {
    let mut height = 0;
    let mut days = 0;
    loop {
        height += up_speed;
        if height >= desired_height {
            return days + 1;
        }
        height -= down_speed;
        days += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::growing_plant::growing_plant;

    struct TestCase {
        up_speed: u32,
        down_speed: u32,
        desired_height: u32,
        want: u32,
    }

    #[test]
    fn test_growing_plant() {
        let test_cases = vec![
            TestCase {
                up_speed: 100,
                down_speed: 10,
                desired_height: 910,
                want: 10,
            },
            TestCase {
                up_speed: 10,
                down_speed: 9,
                desired_height: 4,
                want: 1,
            },
            TestCase {
                up_speed: 5,
                down_speed: 2,
                desired_height: 5,
                want: 1,
            },
            TestCase {
                up_speed: 5,
                down_speed: 2,
                desired_height: 6,
                want: 2,
            },
        ];
        for test_case in test_cases {
            let got = growing_plant(
                test_case.up_speed,
                test_case.down_speed,
                test_case.desired_height,
            );
            assert_eq!(test_case.want, got);
        }
    }
}
