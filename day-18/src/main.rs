use std::usize;

#[derive(PartialEq, Clone, Copy)]
enum MemByte {
    Corrupted,
    Ok,
}

struct Memory {
    width: usize,
    height: usize,
    bytes: Vec<Vec<MemByte>>
}

impl Memory {
    fn new(width: usize, height: usize, input: &str, number_of_fallen_bytes: usize) -> Self {
        let mut bytes = vec![vec![MemByte::Ok; width]; height];

        input.lines().take(number_of_fallen_bytes).for_each(|line| {
            let coords: Vec<usize> = line.split(',').map(|value| value.parse::<usize>().unwrap()).collect();
            bytes[coords[1]][coords[0]] = MemByte::Corrupted;
        });

        Self { width, height, bytes }
    }

    fn compute_steps_to_exit(&self) -> usize {
        let mut distances = vec![vec![usize::MAX; self.width]; self.height];
        let current_distance = 0;
        distances[0][0] = current_distance;
        self.compute_steps_to_exit_rec(1, 0, current_distance + 1, &mut distances);
        self.compute_steps_to_exit_rec(0, 1, current_distance + 1, &mut distances);
        distances[self.height - 1][self.width - 1]
    }

    fn compute_steps_to_exit_rec(&self, x: usize, y: usize, distance: usize, distances: &mut Vec<Vec<usize>>) {
        if self.bytes[y][x] == MemByte::Corrupted {
            return;
        } else {
            if distance >= distances[y][x] {
                return;
            } else {
                distances[y][x] = distance;
                if y > 0 {
                    self.compute_steps_to_exit_rec(x, y-1, distance+1, distances);
                }
                if y < self.height - 1 {
                    self.compute_steps_to_exit_rec(x, y+1, distance+1, distances);
                }
                if x > 0 {
                    self.compute_steps_to_exit_rec(x-1, y, distance+1, distances);
                }
                if x < self.width - 1 {
                    self.compute_steps_to_exit_rec(x+1, y, distance+1, distances);
                }
            }
        }
    }
}

fn get_blocking_byte(width: usize, height: usize, input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut index = lines.len();
    loop {
        let memory = Memory::new(width, height, input, index);
        if memory.compute_steps_to_exit() < usize::MAX {
            break;
        }
        index -= 1;
    }
    lines[index].to_string()
}

fn main() {
    let input = include_str!("../../input/day-18");
    let memory = Memory::new(71, 71, input, 1024);
    let distance = memory.compute_steps_to_exit();
    println!("Distance: {}", distance);

    let blocking_byte = get_blocking_byte(71, 71, input);
    println!("Blocking byte: {}", blocking_byte);
}

#[cfg(test)]
mod tests {
    use crate::{get_blocking_byte, Memory};


    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day-18-test");
        let memory = Memory::new(7, 7, input, 12);
        let distance = memory.compute_steps_to_exit();
        assert_eq!(distance, 22);
    }
    
    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day-18-test");
        let blocking_byte = get_blocking_byte(7, 7, input);
        assert_eq!(blocking_byte, "6,1".to_string());
    }
}