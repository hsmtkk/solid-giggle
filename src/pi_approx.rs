fn pi_approx(epsilon: f64) -> (u64, String) {
    let mut k = 0;
    let mut sum = 0.0;
    loop {
        sum += calculate(k);
        let approx_pi = 4.0 * sum;
        if (approx_pi - std::f64::consts::PI).abs() < epsilon {
            return (k + 1, format!("{:.10}", approx_pi));
        }
        k += 1;
    }
}

fn calculate(k: u64) -> f64 {
    if k % 2 == 0 {
        1.0 / (2 * k + 1) as f64
    } else {
        -1.0 / (2 * k + 1) as f64
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pi_approx() {
        let test_cases = vec![
            (0.1, 10, "3.0418396189"),
            (0.01, 100, "3.1315929036"),
            (0.001, 1000, "3.1405926538"),
            (0.0001, 10000, "3.1414926536"),
            (0.00001, 100001, "3.1416026535"),
            (0.000001, 1000001, "3.1415936536"),
            (0.05, 20, "3.0916238067"),
        ];
        for test_case in test_cases {
            let (got_iter, got_value) = super::pi_approx(test_case.0);
            assert_eq!(test_case.1, got_iter);
            assert_eq!(test_case.2, got_value);
        }
    }
}
