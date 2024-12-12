use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug)]
struct Instructions {
    forbidden_after: HashMap<u32, Vec<u32>>,
    pages: Vec<Pages>,
}

impl Instructions {
    fn sum_valid_pages_numbers(&self) -> u32 {
        self.pages.iter().filter(|pages| pages.is_valid(&self.forbidden_after)).map(|pages| pages.middle_page_number()).sum()
    }

    fn sum_fixed_invalid_pages_numbers(&self) -> u32 {
        let mut invalid_pages: Vec<Pages> = self.pages.iter().filter(|pages| !pages.is_valid(&self.forbidden_after)).cloned().collect();

        invalid_pages.iter_mut().map(|pages| {
            pages.pages.sort_by(|a, b| {
                match self.forbidden_after.get(b) {
                    Some(list) => {
                        if list.contains(a) {
                            Ordering::Less
                        } else {
                            Ordering::Equal
                        }
                    },
                    None => Ordering::Equal,
                }
            });
            pages
        }).map(|pages| pages.middle_page_number()).sum()
    }
}

impl From<&str> for Instructions {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();
        let mut forbidden_after: HashMap<u32, Vec<u32>> = HashMap::new();
        lines.by_ref().take_while(|line| !line.is_empty()).for_each(|line| {
            let numbers = line.split('|').map(|value| value.parse::<u32>().expect("invalid value")).collect::<Vec<u32>>();
            forbidden_after.entry(numbers[1]).and_modify(|rule| rule.push(numbers[0])).or_insert(vec![numbers[0]]);
        });

        let pages = lines.map(Pages::from).collect();
        Self { forbidden_after, pages }
    }
}

#[derive(Debug, Clone)]
struct Pages {
    pub pages: Vec<u32>
}

impl Pages {
    fn is_valid(&self, forbidden_after: &HashMap<u32, Vec<u32>>) -> bool {
        let mut forbidden_pages = vec![];
        for page in &self.pages {
            if forbidden_pages.contains(&page) {
                // println!("{:?} is invalid", self.pages);
                // println!("current number is {page}");
                // println!("current interdictions are {:?}", forbidden_pages);
                return false;
            } else if let Some(forbidden) = forbidden_after.get(page) {
                forbidden.iter().for_each(|f| forbidden_pages.push(f));
            }
        }
        // println!("{:?} is valid", self.pages);
        true
    }

    fn middle_page_number(&self) -> u32 {
        let length = self.pages.len();
        self.pages[length / 2]
    }
}

impl From<&str> for Pages {
    fn from(input: &str) -> Self {
        let pages = input.split(',').map(|value| value.parse::<u32>().expect("invalid value")).collect();
        Self { pages}
    }
}

fn main() {
    let input = include_str!("../../input/day-05");
    let instructions = Instructions::from(input);
    let sum = instructions.sum_valid_pages_numbers();
    println!("Sum is {sum}");

    let sum_invalid = instructions.sum_fixed_invalid_pages_numbers();
    println!("Sum of invalid is {sum_invalid}");
}

#[cfg(test)]
mod tests {
    use crate::Instructions;


    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day-05-test");
        let instructions = Instructions::from(input);
        let sum = instructions.sum_valid_pages_numbers();
        assert_eq!(sum, 143);
    }
    
    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day-05-test");
        let instructions = Instructions::from(input);
        let sum = instructions.sum_fixed_invalid_pages_numbers();
        assert_eq!(sum, 123);
    }
}