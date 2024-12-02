use crate::helpers;
use once_cell::sync::Lazy;
use regex::Regex;

fn load_reports(input_file_name: &str) -> Vec<Vec<u32>> {
    static NUMBERS_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)").unwrap());

    let mut return_value: Vec<Vec<u32>> = Vec::new();

    if let Ok(lines) = helpers::read_lines(input_file_name) {
        for line in lines.flatten() {
            let report: Vec<u32> = NUMBERS_RE.find_iter(&line).map(|value| value.as_str().parse().unwrap()).collect();
            return_value.push(report);
        }
    }

    return_value
}

fn is_safe(report: Vec<u32>) -> bool {

    let mut pos = 1;
    while report[0] == report[pos] {
        pos += 1;
    }

    let is_ascending = report[0] < report[pos];

    let mut idx = 1;

    while idx < report.len() {
            let val1 = report[idx];
            let val2 = report[idx - 1];
            if val1 == val2 {
                return false;
            }
            if (is_ascending && val1 < val2)
            || (!is_ascending && val1 > val2) {
            return false;
        }

        if val1.abs_diff(val2) > 3 {
            return false;
        }
        idx += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_simple() {
        let reports = load_reports("./src/resources/day02_simple.txt");
        println!("{:?}", reports);
    }

    #[test]
    fn test_is_safe() {
        assert_eq!(false, is_safe(vec![9,9,12,9]));
        assert_eq!(true, is_safe(vec![7, 6, 4, 2, 1]));
        assert_eq!(false, is_safe(vec![1, 2, 7, 8, 9]));
        assert_eq!(false, is_safe(vec![9, 7, 6, 2, 1]));
        assert_eq!(false, is_safe(vec![1, 3, 2, 4, 1]));
        assert_eq!(false, is_safe(vec![8, 6, 4, 4, 1]));
        assert_eq!(true, is_safe(vec![1, 3, 6, 7, 9]));
        assert_eq!(false, is_safe(vec![8, 11, 14, 16, 15]));
    }

    #[test]
    fn test_simple_sum() {
        let reports = load_reports("./src/resources/day02_simple.txt");
        let safe_report_count = reports.iter().filter(|report| is_safe(report.to_vec())).count();
        println!("Count {}", safe_report_count);
        assert_eq!(2, safe_report_count);

    }

    #[test]
    fn test_sum() {
        let reports = load_reports("./src/resources/day02_input.txt");

        let safe_report_count = reports.iter().filter(|report| is_safe(report.to_vec())).count();
        println!("Count {}", safe_report_count);
        assert_eq!(472, safe_report_count);
    }

}
