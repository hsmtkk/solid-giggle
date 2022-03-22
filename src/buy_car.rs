struct Parameter {
    start_price_old: u32,
    start_price_new: u32,
    saving_per_month: u32,
    percent_loss_by_month: f64,
}

impl Parameter {
    fn new(
        start_price_old: u32,
        start_price_new: u32,
        saving_per_month: u32,
        percent_loss_by_month: f64,
    ) -> Self {
        Parameter {
            start_price_old,
            start_price_new,
            saving_per_month,
            percent_loss_by_month,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Answer {
    month: u32,
    available: u32,
}

impl Answer {
    fn new(month: u32, available: u32) -> Self {
        Answer { month, available }
    }
}

fn nb_months(param: Parameter) -> Answer {
    if param.start_price_old >= param.start_price_new {
        return Answer::new(0, param.start_price_old - param.start_price_new);
    }
    let mut old = param.start_price_old as f64;
    let mut new = param.start_price_new as f64;
    let mut loss_percent = param.percent_loss_by_month;
    let mut month = 1;
    loop {
        if month % 2 == 0 {
            loss_percent = param.percent_loss_by_month + 0.5 * (month / 2) as f64;
        }
        old *= 1.0 - 0.01 * loss_percent;
        new *= 1.0 - 0.01 * loss_percent;
        let saving = (param.saving_per_month * month) as f64;
        let diff = old + saving - new;
        if diff > 0.0 {
            return Answer::new(month, diff.round() as u32);
        }
        month += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{nb_months, Answer, Parameter};

    #[test]
    fn test0() {
        let want = Answer::new(6, 766);
        let got = nb_months(Parameter::new(2000, 8000, 1000, 1.5));
        assert_eq!(want, got);
    }

    #[test]
    fn test1() {
        let want = Answer::new(0, 4000);
        let got = nb_months(Parameter::new(12000, 8000, 1000, 1.5));
        assert_eq!(want, got);
    }

    #[test]
    fn test2() {
        let want = Answer::new(8, 597);
        let got = nb_months(Parameter::new(8000, 12000, 500, 1.0));
        assert_eq!(want, got);
    }

    #[test]
    fn test3() {
        let want = Answer::new(8, 332);
        let got = nb_months(Parameter::new(18000, 32000, 1500, 1.25));
        assert_eq!(want, got);
    }

    #[test]
    fn test4() {
        let want = Answer::new(25, 122);
        let got = nb_months(Parameter::new(7500, 32000, 300, 1.55));
        assert_eq!(want, got);
    }

    #[test]
    fn test5() {
        let want = Answer::new(1, 110);
        let got = nb_months(Parameter::new(2600, 3500, 1000, 1.2));
        assert_eq!(want, got);
    }
}
