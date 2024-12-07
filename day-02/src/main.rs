#[derive(Debug, Clone)]
struct Report {
    levels: Vec<i32>
}

impl Report {
    fn is_safe(&self) -> bool {
        is_levels_vec_safe(&self.levels)
    }

    fn is_safe_with_dampener(&self) -> bool {
        if is_levels_vec_safe(&self.levels) {
            true
        } else {
            for i in 0..self.levels.len() {
                let mut shorten_levels = self.levels.clone();
                shorten_levels.remove(i);
                if is_levels_vec_safe(&shorten_levels) {
                    return true;
                }
            }
            false
        }
    }
}

impl From<&str> for Report {
    fn from(value: &str) -> Self {
        let levels = value.split(' ').map(|v| v.parse::<i32>().unwrap()).collect();
        Report{ levels }
    }
}

fn is_levels_vec_safe(levels: &[i32]) -> bool {
    let differences: Vec<i32> = levels.windows(2).map(|w| {
        w[0] - w[1]
    }).collect();
    let is_decreasing_within_limits = differences.iter().all(|diff| diff > &0 && diff < &4);
    let is_increasing_within_limits = differences.iter().all(|diff| diff < &0 && diff > &-4);
    is_decreasing_within_limits || is_increasing_within_limits
}

fn create_reports(input: &str) -> Vec<Report> {
    input.lines().map(|line| line.into()).collect()
}

fn count_safe_reports(reports: Vec<Report>) -> usize {
    reports.iter().filter(|report| report.is_safe()).count()
}

fn count_safe_reports_with_dampener(reports: Vec<Report>) -> usize {
    reports.iter().filter(|report| report.is_safe_with_dampener()).count()
}

fn main() {
    let input = include_str!("../../input/day-02");
    let reports = create_reports(input);
    let number_of_safe_reports = count_safe_reports(reports.clone());
    println!("Number of safe reports: {number_of_safe_reports}");

    let number_of_safe_reports_with_dampener = count_safe_reports_with_dampener(reports);
    println!("Number of safe reports with Dampener: {number_of_safe_reports_with_dampener}");
}

#[cfg(test)]
mod tests {
    use crate::{count_safe_reports, count_safe_reports_with_dampener, create_reports};

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day-02-test");
        let reports = create_reports(input);
        let number_of_safe_reports = count_safe_reports(reports);
        assert_eq!(number_of_safe_reports, 2);
    }
    
    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day-02-test");
        let reports = create_reports(input);
        let number_of_safe_reports = count_safe_reports_with_dampener(reports);
        assert_eq!(number_of_safe_reports, 4);
    }
}