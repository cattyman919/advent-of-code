use std::path::Path;

use crate::utils;

type Report = Vec<u32>;

fn is_report_safe(report: &Report) -> bool {
    if report.len() < 2 {
        return false;
    }

    let current = report[0];
    let next = report[1];

    // 1. Check the first difference
    let first_diff = current.abs_diff(next);
    if !(1..=3).contains(&first_diff) {
        return false;
    }

    let increasing = current < next;

    report.windows(2).skip(1).all(|window| {
        let current = window[0];
        let next = window[1];

        // 1. Check the first difference
        let first_diff = current.abs_diff(next);
        if !(1..=3).contains(&first_diff) {
            return false;
        }

        let current_is_increasing = current < next;

        if increasing != current_is_increasing {
            return false;
        }

        true
    })
}

pub fn run<P: AsRef<Path>>(path: P) -> u32 {
    let list_report_string = utils::read_file(path);

    let reports: Vec<Report> = list_report_string
        .into_iter()
        .map(|report_string| {
            report_string
                .split_whitespace()
                .map(|level| level.parse().expect("Expect a number"))
                .collect()
        })
        .collect();

    let safe_count = reports
        .iter()
        .filter(|report| is_report_safe(report))
        .count();

    safe_count as u32
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_input() {
        let path = Path::new("input")
            .join("day_2_red_nosed_reports")
            .join("test-input-1.txt");
        assert_eq!(2, run(path))
    }
}
