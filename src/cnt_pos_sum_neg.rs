use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
struct Answer {
    count: u32,
    sum: i32,
}

fn count_positive_sum_negative(ns: &[i32]) -> Answer {
    let mut count = 0;
    let mut sum: i32 = 0;
    for n in ns {
        match n.cmp(&0) {
            Ordering::Greater => {
                count += 1;
            }
            Ordering::Less => {
                sum += n;
            }
            _ => {}
        }
    }
    Answer { count, sum }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let ns = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15];
        let want = super::Answer {
            count: 10,
            sum: -65,
        };
        let got = super::count_positive_sum_negative(&ns);
        assert_eq!(want, got);
    }
}
