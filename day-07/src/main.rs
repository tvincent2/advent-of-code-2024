#[derive(Debug)]
struct Equation {
    result: u64,
    numbers: Vec<u64>
}

impl Equation {
    fn is_valid(&self) -> bool {
        let index = 1;
        let init = self.numbers[0];
        self.is_valid_rec(index, init + self.numbers[index]) || self.is_valid_rec(index, init * self.numbers[index])
    }

    fn is_valid_rec(&self, index: usize, value: u64) -> bool {
        if index == self.numbers.len() - 1 {
            value == self.result
        } else if value > self.result {
            false
        } else {
            let new_index = index + 1;
            let new_value_add = value + self.numbers[new_index];
            let new_value_mul = value * self.numbers[new_index];
            self.is_valid_rec(new_index, new_value_add) || self.is_valid_rec(new_index, new_value_mul)
        }
    }

    fn is_valid_with_concatenation(&self) -> bool {
        let index = 1;
        let init = self.numbers[0];
        let next_number = self.numbers[index];
        self.is_valid_with_concatenation_rec(index, init + next_number) || self.is_valid_with_concatenation_rec(index, init * next_number) || self.is_valid_with_concatenation_rec(index, concat(init, next_number))
    }

    fn is_valid_with_concatenation_rec(&self, index: usize, value: u64) -> bool {
        if index == self.numbers.len() - 1 {
            value == self.result
        } else if value > self.result {
            false
        } else {
            let new_index = index + 1;
            let next_number = self.numbers[new_index];
            let new_value_add = value + next_number;
            let new_value_mul = value * next_number;
            let new_value_concat = concat(value, next_number);
            self.is_valid_with_concatenation_rec(new_index, new_value_add) || self.is_valid_with_concatenation_rec(new_index, new_value_mul) || self.is_valid_with_concatenation_rec(new_index, new_value_concat)
        }
    }
}

fn concat(a: u64, b: u64) -> u64 {
    // because I can't to be seem able to use math for this
    let mut s = a.to_string();
    s.push_str(&b.to_string());
    s.parse::<u64>().unwrap()
}

impl From<&str> for Equation {
    fn from(input: &str) -> Self {
        let mut split_result = input.split(": ");
        let result = split_result.next().unwrap().parse::<u64>().unwrap();
        let numbers = split_result.next().unwrap().split(' ').map(|number| number.parse::<u64>().unwrap()).collect();
        Self { result, numbers }
    }
}

struct Equations {
    equations: Vec<Equation>,
}

impl Equations {
    fn calibration_result(&self) -> u64 {
        self.equations.iter().filter(|equation| equation.is_valid()).map(|equation| equation.result).sum()
    }

    fn calibration_result_part2(&self) -> u64 {
        self.equations.iter().filter(|equation| equation.is_valid_with_concatenation()).map(|equation| equation.result).sum()
    }
}

impl From<&str> for Equations {
    fn from(input: &str) -> Self {
        let equations = input.lines().map(|line| Equation::from(line)).collect();
        Self { equations }
    }
}

fn main() {
    let input = include_str!("../../input/day-07");
    let equations = Equations::from(input);
    let calibration_result = equations.calibration_result();
    println!("calibration result: {calibration_result}");

    let calibration_result_with_concat = equations.calibration_result_part2();
    println!("calibration result with concat: {calibration_result_with_concat}");
}

#[cfg(test)]
mod tests {
    use crate::Equations;


    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day-07-test");
        let equations = Equations::from(input);
        let calibration_result = equations.calibration_result();
        assert_eq!(calibration_result, 3749);
    }
    
    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day-07-test");
        let equations = Equations::from(input);
        let calibration_result = equations.calibration_result_part2();
        assert_eq!(calibration_result, 11387);
    }
}