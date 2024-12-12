fn trim_until_mul(input: String) -> Option<String> {
    input.find("mul(").map(|index| input[(index+4)..].to_string())
}

fn process_multiplication_candidate(input: String) -> (u64, String) {
    if let Some(comma_index) = input.find(',') {
        if let Some(mul_index) = input.find("mul(") {
            if mul_index < comma_index {
                let next_input = input[mul_index..].to_string();
                println!("failed to find a multiplication end at {}", &input[0..comma_index]);
                return (0, next_input);
            }
        }
        let forward_input = input[(comma_index+1)..].to_string();
        let factor1 = match input[0..comma_index].parse::<u64>() {
            Ok(number) => number,
            Err(_) => {
                println!("failed to find a first factor at {}", &input[0..comma_index]);
                return (0, forward_input);
            },
        };
        if let Some(par_index) = forward_input.find(')') {
            if let Some(mul_index) = forward_input.find("mul(") {
                if mul_index < par_index {
                    let next_input = forward_input[mul_index..].to_string();
                    println!("failed to find a multiplication end at {}", &forward_input[0..par_index]);
                    return (0, next_input);
                }
            }
            let forward_input2 = forward_input[(par_index+1)..].to_string();
            let factor2 = match forward_input[0..par_index].parse::<u64>() {
                Ok(number) => number,
                Err(_) => {
                    println!("failed to find a second factor at {}", &forward_input[0..par_index]);
                    return (0, forward_input2);
                },
            };
            (factor1*factor2, forward_input2)
        } else {
            (0, "".to_string())
        }
    }
    else {
        (0, "".to_string())
    }
}

fn multiply(mut input: String) -> u64 {
    let mut result = 0;
    let mut count = 0;
    while let Some(inp) = trim_until_mul(input) {
        let (number, new_input) = process_multiplication_candidate(inp);
        if number > 0 {
            count += 1;
        }
        result += number;
        input = new_input;
    }
    println!("{count} multiplications");
    result
}

fn multiply_enabled_only(input: String) -> u64 {
    if let Some(first_dont) = input.find("don't()") {
        let first_multiply = multiply(input[..first_dont].to_string());
        first_multiply + input.split("don't()").map(|inp| {
            if let Some(index) = inp.find("do()") {
                multiply(inp[(index + 4)..].to_string())
            } else {
                0
            }
        }).sum::<u64>()
    } else {
        multiply(input)
    }
}

fn main() {
    let input = include_str!("../../input/day-03");
    let result = multiply(input.to_string());
    println!("Result part 1: {result}");

    let result = multiply_enabled_only(input.to_string());
    println!("Result part 2: {result}");
}

#[cfg(test)]
mod tests {
    use crate::{multiply, multiply_enabled_only};


    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day-03-test");
        let result = multiply(input.to_string());
        assert_eq!(result, 161);
    }

    #[test]
    fn test_invalid() {
        let invalid_input = "mul(4*";
        let result = multiply(invalid_input.to_string());
        assert_eq!(result, 0);

        let invalid_input = "mul(6,9!";
        let result = multiply(invalid_input.to_string());
        assert_eq!(result, 0);

        let invalid_input = "?(12,34)";
        let result = multiply(invalid_input.to_string());
        assert_eq!(result, 0);

        let invalid_input = "mul ( 2 , 4 )";
        let result = multiply(invalid_input.to_string());
        assert_eq!(result, 0);

        let invalid_input = "mul( 2 , 4 )";
        let result = multiply(invalid_input.to_string());
        assert_eq!(result, 0);
    }

    #[test]
    fn test_adjacent_muls() {
        let input = "mul(6,9)mul(23,2)";
        let result = multiply(input.to_string());
        assert_eq!(result, 100);
    }
    
    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day-03-test-part2");
        let result = multiply_enabled_only(input.to_string());
        assert_eq!(result, 48);
    }
}