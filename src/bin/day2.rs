fn main() {
    let example_data = include_str!("../input/day2_example.txt");
    let data = include_str!("../input/day2.txt");

    let example_reports = parse_reports(example_data);
    let reports = parse_reports(data);

    assert_eq!(count_valid_reports(&example_reports), 2);
    println!("1: {}", count_valid_reports(&reports));

    assert_eq!(count_valid_reports_with_leniency(&example_reports), 4);
    println!("2: {}", count_valid_reports_with_leniency(&reports));
}

fn parse_reports(data: &str) -> Vec<Vec<usize>> {
    data.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_report_safe(report: &[usize]) -> bool {
    let mut is_increasing = None;

    for window in report.windows(2) {
        let (prev, curr) = (window[0], window[1]);

        if prev.abs_diff(curr) < 1 || prev.abs_diff(curr) > 3 {
            return false;
        }

        match is_increasing {
            None => is_increasing = Some(prev <= curr),
            Some(true) if prev > curr => return false,
            Some(false) if prev < curr => return false,
            _ => {}
        }
    }

    true
}

fn count_valid_reports(reports: &[Vec<usize>]) -> usize {
    reports.iter().filter(|r| is_report_safe(r)).count()
}

fn count_valid_reports_with_leniency(reports: &[Vec<usize>]) -> usize {
    reports
        .iter()
        .filter(|report| {
            (0..report.len()).any(|i| {
                let mut modified = report.to_vec();
                modified.remove(i);
                is_report_safe(&modified)
            })
        })
        .count()
}
