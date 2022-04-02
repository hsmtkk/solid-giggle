fn tribonacci(input: (u32, u32, u32), length: usize) -> Option<Vec<u32>> {
    if length < 3 {
        return None;
    }
    let mut x = input.0;
    let mut y = input.1;
    let mut z = input.2;
    let mut result = vec![x, y, z];
    for _i in 0..length {
        let newx = y;
        let newy = z;
        let newz = x + y + z;
        result.push(newz);
        x = newx;
        y = newy;
        z = newz;
    }
    result = result[0..length].to_vec();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::tribonacci;

    struct TestCase {
        input: (u32, u32, u32),
        length: usize,
        want: Option<Vec<u32>>,
    }

    impl TestCase {
        fn new(input: (u32, u32, u32), length: usize, want: Option<&[u32]>) -> Self {
            match want {
                Some(want) => TestCase {
                    input,
                    length,
                    want: Some(want.to_vec()),
                },
                None => TestCase {
                    input,
                    length,
                    want: None,
                },
            }
        }
    }

    #[test]
    fn test0() {
        let test_cases = vec![
            TestCase::new(
                (1, 1, 1),
                10,
                Some(&vec![1, 1, 1, 3, 5, 9, 17, 31, 57, 105]),
            ),
            TestCase::new((0, 1, 1), 10, Some(&vec![0, 1, 1, 2, 4, 7, 13, 24, 44, 81])),
            TestCase::new((300, 200, 100), 0, None),
        ];
        for test_case in test_cases {
            let got = tribonacci(test_case.input, test_case.length);
            assert_eq!(test_case.want, got);
        }
    }
}
