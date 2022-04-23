fn sjf(jobs: &[usize], index: usize) -> usize {
    let mut jobs = jobs.to_vec();
    let mut cc = 0;
    loop {
        let shortest_index = find_shortest_job(&jobs);
        jobs[shortest_index] -= 1;
        cc += 1;
        if jobs[index] == 0 {
            break;
        }
    }
    cc
}

fn find_shortest_job(jobs: &[usize]) -> usize {
    let mut shortest = *jobs.iter().max().unwrap();
    let mut shortest_index = 0;
    for (i, &job) in jobs.iter().enumerate() {
        if job == 0 {
            continue;
        }
        if job < shortest {
            shortest = job;
            shortest_index = i;
        }
    }
    shortest_index
}

#[cfg(test)]
mod tests {
    use super::sjf;

    #[test]
    fn returns_expected() {
        assert_eq!(sjf(&[100], 0), 100);
        assert_eq!(sjf(&[3, 10, 20, 1, 2], 0), 6);
    }
}
