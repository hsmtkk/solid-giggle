fn calc(array: Vec<i32>) -> i32 {
    array.iter().map(square_if_greater_than_zero).enumerate().map(multiple3_third).enumerate().map(multiple1_fifth).sum()
}

fn square_if_greater_than_zero(x:&i32) -> i32 {
    let x = *x;
    if x > 0 {
        x * x
    } else {
        x
    }
}

fn multiple3_third(elem:(usize, i32)) -> i32 {
    let index = elem.0;
    let value = elem.1;
    if (index + 1)% 3 == 0 {
        value * 3
    }  else {
        value
    }
}

fn multiple1_fifth(elem:(usize, i32)) -> i32 {
    let index = elem.0;
    let value = elem.1;
    if (index+1) % 5 == 0 {
        -1 * value
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(calc(vec![0, 2, 1, -6, -3, 3]), 31);
        assert_eq!(calc(vec![0]), 0);
        assert_eq!(calc(vec![1, 1, 1, 1, 1]), 5);
        assert_eq!(calc(vec![1, 1, -9, 9, 16, -15, -45, -73, 26]), 1665);
        assert_eq!(calc(vec![1, -1, 10, -9, 16, 15, 45, -73, -26]), 2584);
        assert_eq!(calc(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 0);
        assert_eq!(calc(vec![-5, -5, -5, -5, -5, -5, -5]), -45);
    }
}