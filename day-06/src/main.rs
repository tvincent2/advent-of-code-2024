#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Guard {
    position: Position,
    direction: Direction,
}

#[derive(Debug, Clone)]
struct LabMap {
    obstacles: Vec<Position>,
    guard_start: Guard,
    height: usize,
    width: usize,
}

impl LabMap {
    fn next_obstacle(&self, start_position: Position, direction: Direction) -> Option<Position> {
        match direction {
            Direction::Up => self.obstacles.iter().filter(|position| start_position.x == position.x && position.y < start_position.y).max_by_key(|position| position.y).copied(),
            Direction::Right => self.obstacles.iter().filter(|position| start_position.y == position.y && position.x > start_position.x).min_by_key(|position| position.x).copied(),
            Direction::Down => self.obstacles.iter().filter(|position| start_position.x == position.x && position.y > start_position.y).min_by_key(|position| position.y).copied(),
            Direction::Left => self.obstacles.iter().filter(|position| start_position.y == position.y && position.x < start_position.x).max_by_key(|position| position.x).copied(),
        }
    }

    fn next(&self, guard: Guard) -> (Vec<Position>, Option<Guard>) {
        let next_obstacle = self.next_obstacle(guard.position, guard.direction);
        let next_direction = guard.direction.turn();

        match guard.direction {
            Direction::Up => {
                let next_guard_y = if let Some(position) = next_obstacle { position.y + 1 } else { 0 };
                let visited_cells = (next_guard_y..=guard.position.y).map(|y| Position{x: guard.position.x, y}).collect();
                let next_guard = next_obstacle.map(|_| Guard{ direction: next_direction, position: Position { x: guard.position.x, y: next_guard_y }});
                (visited_cells, next_guard)
            },
            Direction::Right => {
                let next_guard_x = if let Some(position) = next_obstacle { position.x - 1 } else { self.width - 1 };
                let visited_cells = (guard.position.x..=next_guard_x).map(|x| Position{x, y: guard.position.y}).collect();
                let next_guard = next_obstacle.map(|_| Guard{ direction: next_direction, position: Position { x: next_guard_x, y: guard.position.y }});
                (visited_cells, next_guard)
            },
            Direction::Down => {
                let next_guard_y = if let Some(position) = next_obstacle { position.y - 1 } else { self.height - 1 };
                let visited_cells = (guard.position.y..=next_guard_y).map(|y| Position{x: guard.position.x, y}).collect();
                let next_guard = next_obstacle.map(|_| Guard{ direction: next_direction, position: Position { x: guard.position.x, y: next_guard_y }});
                (visited_cells, next_guard)
            },
            Direction::Left => {
                let next_guard_x = if let Some(position) = next_obstacle { position.x + 1 } else { 0 };
                let visited_cells = (next_guard_x..=guard.position.x).map(|x| Position{x, y: guard.position.y}).collect();
                let next_guard = next_obstacle.map(|_| Guard{ direction: next_direction, position: Position { x: next_guard_x, y: guard.position.y }});
                (visited_cells, next_guard)
            },
        }
    }

    fn count_positions(&self) -> (usize, Vec<Position>) {
        let mut guard = self.guard_start;
        let mut visited_positions = vec![self.guard_start.position];
        loop {
            let (mut new_positions, new_guard) = self.next(guard);
            visited_positions.append(&mut new_positions);
            if let Some(new_guard) = new_guard {
                guard = new_guard;
            } else {
                break;
            }
        }
        visited_positions.sort();
        visited_positions.dedup();
        let count = visited_positions.iter().count();
        (count, visited_positions)
    }

    fn does_cycle(&self) -> bool {
        let mut visited_positions = vec![];
        let mut guard = self.guard_start;
        loop {
            let (new_positions, new_guard) = self.next(guard);
            for position in new_positions {
                let pos_with_dir = (position, guard.direction);
                if visited_positions.contains(&pos_with_dir) {
                    return true;
                } else {
                    visited_positions.push(pos_with_dir);
                }
            }
            if let Some(new_guard) = new_guard {
                guard = new_guard;
            } else {
                break;
            }
        }
        false
    }

    fn brute_force_obstructions(&self, visited: Vec<Position>) -> usize {
        let mut map = self.clone();
        let start_position = self.guard_start.position;
        visited.iter().filter(|&&position| position != start_position)
        .filter(|&&position| {
            map.obstacles.push(position);
            let does_cycle = map.does_cycle();
            map.obstacles.pop();
            does_cycle
        }).count()
    }
}

impl From<&str> for LabMap {
    fn from(input: &str) -> Self {
        let height = input.lines().fold(0, |acc, _| acc + 1);
        let width = input.lines().next().unwrap().len();
        let obstacles = input.lines().enumerate().map(|(line_index, line)| 
            line.char_indices().filter(|(_, c)| c == &'#').map(move |(char_index, _)| Position { x: char_index, y: line_index })
        ).flatten().collect::<Vec<Position>>();
        let (mut char_index, mut line_index) = (0, 0);
        for (l_index,line) in input.lines().enumerate() {
            match line.find('^') {
                Some(c_index) => {
                    char_index = c_index;
                    line_index = l_index;
                    break;
                },
                None => continue,
            }
        };
        LabMap { obstacles, guard_start: Guard { position: Position { x: char_index, y: line_index }, direction: Direction::Up }, height, width }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn main() {
    let input = include_str!("../../input/day-06");
    let lab_map = LabMap::from(input);
    let (count, visited) = lab_map.count_positions();
    println!("Positions: {count}");

    let obstructions = lab_map.brute_force_obstructions(visited);
    println!("Obstructions: {obstructions}");
}

#[cfg(test)]
mod tests {
    use crate::LabMap;


    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day-06-test");
        let lab_map = LabMap::from(input);
        let (count, _) = lab_map.count_positions();
        assert_eq!(count, 41);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day-06-test");
        let lab_map = LabMap::from(input);
        let (_, visited) = lab_map.count_positions();
        let count = lab_map.brute_force_obstructions(visited);
        assert_eq!(count, 6);
    }
}