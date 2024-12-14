const MAX_BUTTON_PRESSES: i64 = 100;
const PART_2_OFFSET: i64 = 10_000_000_000_000;

#[derive(Debug)]
struct Machines {
    machines: Vec<Machine>,
}

impl Machines {
    fn count_tokens_bruteforce(&self) -> i64 {
        self.machines.iter().filter_map(|machine| machine.count_tokens_bruteforce()).sum()
    }

    fn count_tokens(&self) -> i64 {
        self.machines.iter().filter_map(|machine| machine.count_tokens()).sum()
    }

    fn add_offset(&self) -> Self {
        let machines = self.machines.iter().map(|machine| machine.add_offset()).collect();
        Self { machines }
    }
}

impl From<&str> for Machines {
    fn from(input: &str) -> Self {
        let mut machines = vec![];

        let mut lines = input.lines();
        while let Some(line_a) = lines.next() {
            let mut split = line_a.split(',');
            let a_x = split.next().unwrap().split('+').skip(1).next().unwrap().parse::<i64>().unwrap();
            let a_y = split.next().unwrap().split('+').skip(1).next().unwrap().parse::<i64>().unwrap();
    
            let line_b = lines.next().unwrap();
            let mut split = line_b.split(',');
            let b_x = split.next().unwrap().split('+').skip(1).next().unwrap().parse::<i64>().unwrap();
            let b_y = split.next().unwrap().split('+').skip(1).next().unwrap().parse::<i64>().unwrap();
    
            let line_prize = lines.next().unwrap();
            let mut split = line_prize.split(',');
            let prize_x = split.next().unwrap().split('=').skip(1).next().unwrap().parse::<i64>().unwrap();
            let prize_y = split.next().unwrap().split('=').skip(1).next().unwrap().parse::<i64>().unwrap();
    
            machines.push(Machine { a_x, a_y, b_x, b_y, prize_x, prize_y });
            lines.next();
        }

        Self { machines }
    }
}

#[derive(Debug)]
struct Machine {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    prize_x: i64,
    prize_y: i64,
}

impl Machine {
    fn count_tokens_bruteforce(&self) -> Option<i64> {
        // worst case is 100 times button A -> 300 tokens and 100 times button B, +1 to make it infeasible
        let upper_bound = 401;
        let mut minimum_tokens = upper_bound;
        for a_presses in 0..MAX_BUTTON_PRESSES {
            for b_presses in 0..MAX_BUTTON_PRESSES {
                let x_position = a_presses * self.a_x + b_presses * self.b_x;
                let y_position = a_presses * self.a_y + b_presses * self.b_y;
                let tokens = a_presses * 3 + b_presses;
                if x_position == self.prize_x && y_position == self.prize_y && tokens < minimum_tokens {
                    minimum_tokens = tokens;
                }
            }
        }
        if minimum_tokens >= upper_bound {
            None
        } else {
            Some(minimum_tokens)
        }
    }

    fn count_tokens(&self) -> Option<i64> {
        // we're actually solving a system of 2 equations with 2 unknowns
        let determinant = self.a_x * self.b_y - self.b_x * self.a_y;
        if determinant == 0 {
            None
        } else {
            let determinant_1 = self.prize_x * self.b_y - self.b_x * self.prize_y;
            let determinant_2 = self.a_x * self.prize_y - self.prize_x * self.a_y;
            let a = determinant_1 / determinant;
            let b = determinant_2 / determinant;

            if self.a_x * a + self.b_x * b == self.prize_x && self.a_y * a + self.b_y * b == self.prize_y {
                Some(3 * a + b)
            } else {
                None
            }
        }
    }

    fn add_offset(&self) -> Self {
        Self { a_x: self.a_x, a_y: self.a_y, b_x: self.b_x, b_y: self.b_y, prize_x: self.prize_x + PART_2_OFFSET, prize_y: self.prize_y + PART_2_OFFSET }
    }
}

fn main() {
    let input = include_str!("../../input/day-13");
    let machines = Machines::from(input);
    let tokens = machines.count_tokens_bruteforce();
    println!("Tokens: {}", tokens);

    let machines_with_offset = machines.add_offset();
    let tokens_with_offset = machines_with_offset.count_tokens();
    println!("Tokens with offset: {}", tokens_with_offset);
}

#[cfg(test)]
mod tests {
    use crate::Machines;


    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day-13-test");
        let machines = Machines::from(input);
        let tokens = machines.count_tokens_bruteforce();
        assert_eq!(tokens, 480);
    }
}