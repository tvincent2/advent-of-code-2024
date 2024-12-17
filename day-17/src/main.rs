struct Computer {
    ra: u64,
    rb: u64,
    rc: u64,
    instructions: Vec<u8>,
}

impl Computer {
    fn get_combo(&self, combo: u8) -> u64 {
        match combo {
            0 | 1 | 2 | 3 => combo as u64,
            4 => self.ra,
            5 => self.rb,
            6 => self.rc,
            _ => unreachable!()
        }
    }

    fn run_program(&mut self) -> Vec<String> {
        let mut outputs: Vec<String> = vec![];
        let mut pointer = 0;

        while pointer < self.instructions.len() {
            let op = self.instructions[pointer];
            let operand = self.instructions[pointer + 1];
            match op {
                0 => { // adv
                    let denominator = 2u64.pow(self.get_combo(operand) as u32);
                    self.ra /= denominator;
                    pointer += 2;
                },
                1 => { // bxl
                    self.rb = self.rb ^ operand as u64;
                    pointer += 2;
                },
                2 => { // bst
                    self.rb = self.get_combo(operand) % 8;
                    pointer += 2;
                },
                3 => { // jnz
                    if self.ra == 0 {
                        pointer += 2;
                    } else {
                        pointer = operand as usize;
                    }
                },
                4 => { // bxc
                    self.rb = self.rb ^ self.rc;
                    pointer += 2;
                },
                5 => { // out
                    let value_to_push = self.get_combo(operand) % 8;
                    outputs.push(value_to_push.to_string());
                    pointer += 2;
                },
                6 => { // bdv
                    let denominator = 2u64.pow(self.get_combo(operand) as u32);
                    self.rb = self.ra / denominator;
                    pointer += 2;
                },
                7 => { // cdv
                    let denominator = 2u64.pow(self.get_combo(operand) as u32);
                    self.rc = self.ra / denominator;
                    pointer += 2;
                },
                _ => unreachable!(),
            }
        }
        outputs
    }

    fn find_register(&mut self) -> u64 {
        let instructions_strings: Vec<String> = self.instructions.iter().map(|i| i.to_string()).collect();
        let instructions = instructions_strings;
        let mut ra = 8u64.pow(15);

        loop {
            self.ra = ra;
            self.rb = 0;
            self.rc = 0;
            let output = self.run_program();

            if output == instructions {
                return ra;
            } else {
                assert_eq!(output.len(), instructions.len());
                let len = output.len();
                let mut i = 0;
                while output[len - i - 1] == instructions[len -i - 1] {
                    i+=1;
                }
                ra += 8u64.pow((len-i-1) as u32);
            }
        }
    }
}

impl From<&str> for Computer {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();
        let ra = lines.next().unwrap()[12..].parse::<u64>().unwrap();
        let rb = lines.next().unwrap()[12..].parse::<u64>().unwrap();
        let rc = lines.next().unwrap()[12..].parse::<u64>().unwrap();
        lines.next();
        let instructions = lines.next().unwrap()[9..].split(',').map(|c| c.parse::<u8>().unwrap()).collect();

        Self { ra, rb, rc, instructions }
    }
}

fn main() {
    let input = include_str!("../../input/day-17");
    let mut computer = Computer::from(input);
    let output = computer.run_program();
    println!("Program output: {}", output.join(","));

    let ra = computer.find_register();
    println!("Register value: {}", ra);
}

// 35184372088832

#[cfg(test)]
mod tests {
    use crate::Computer;

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day-17-test");
        let mut computer = Computer::from(input);
        let output = computer.run_program();
        assert_eq!(output.join(","), "4,6,3,5,6,3,5,2,1,0".to_string());
    }
    
    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day-17-test-part2");
        let mut computer = Computer::from(input);
        let ra = computer.find_register();
        assert_eq!(ra, 117440);
    }
}