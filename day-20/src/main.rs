use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy)]
enum Cell {
    Wall,
    Empty,
    Start,
    End,
}

impl Cell {
    fn is_on_path(&self) -> bool {
        match self {
            Cell::Empty | Cell::End => true,
            _ => false,
        }
    }
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            '.' => Self::Empty,
            'S' => Self::Start,
            'E' => Self::End,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn up(&self) -> Self {
        Position { x: self.x, y: self.y-1 }
    }

    fn down(&self) -> Self {
        Position { x: self.x, y: self.y+1 }
    }

    fn left(&self) -> Self {
        Position { x: self.x-1, y: self.y }
    }

    fn right(&self) -> Self {
        Position { x: self.x+1, y: self.y }
    }

    fn distance(&self, other: &Position) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
 }

struct Cpu {
    cells: Vec<Vec<Cell>>,
    start: Position,
    end: Position,
    height: usize,
    width: usize,
}

impl From<&str> for Cpu {
    fn from(input: &str) -> Self {
        let cells: Vec<Vec<Cell>> = input.lines()
            .map(|line| line.chars().map(Cell::from).collect()).collect();

        let start = cells.iter().enumerate().filter_map(|(y,row)| row.iter().position(|cell| cell == &Cell::Start).map(|x| Position {x, y})).next().unwrap();
        let end = cells.iter().enumerate().filter_map(|(y,row)| row.iter().position(|cell| cell == &Cell::End).map(|x| Position {x, y})).next().unwrap();

        let height = cells.len();
        let width = cells[0].len();

        Self { cells, start, end, height, width }
    }
}


impl Cpu {
    fn cell_at(&self, position: Position) -> Cell {
        self.cells[position.y][position.x]
    }

    fn compute_path(&self) -> (Vec<Position>, HashMap<Position, u32>) {
        let mut distance_from_start: HashMap<Position, u32> = [(self.start, 0)].into();
        let mut positions = vec![self.start];

        let mut previous_position = self.start;
        let mut current_position = self.find_first_move();
        let mut distance = 1;
        positions.push(current_position);
        distance_from_start.insert(current_position, distance);

        while current_position != self.end {
            let next_position = self.find_next_move(current_position, previous_position);
            previous_position = current_position;
            current_position = next_position;
            distance += 1;
            positions.push(current_position);
            distance_from_start.insert(current_position, distance);
        }
        
        (positions, distance_from_start)
    }

    fn compute_shortcuts(&self, path: Vec<Position>, distances: HashMap<Position, u32>) -> HashMap<(Position, Direction), u32> {
        let mut shortcuts = HashMap::new();
        for position in path {
            let distance = distances.get(&position).unwrap();
            if position.y > 1 {
                let up2 = position.up().up();
                if let Some(other_distance) = distances.get(&up2) {
                    if distance + 2 < *other_distance {
                        shortcuts.insert((position, Direction::Up), other_distance - distance - 2);
                    }
                }
            }
            if position.y < self.height - 2 {
                let down2 = position.down().down();
                if let Some(other_distance) = distances.get(&down2) {
                    if distance + 2 < *other_distance {
                        shortcuts.insert((position, Direction::Down), other_distance - distance - 2);
                    }
                }
            }
            if position.x > 1 {
                let left2 = position.left().left();
                if let Some(other_distance) = distances.get(&left2) {
                    if distance + 2 < *other_distance {
                        shortcuts.insert((position, Direction::Left), other_distance - distance - 2);
                    }
                }
            }
            if position.x < self.width - 2 {
                let right2 = position.right().right();
                if let Some(other_distance) = distances.get(&right2) {
                    if distance + 2 < *other_distance {
                        shortcuts.insert((position, Direction::Right), other_distance - distance - 2);
                    }
                }
            }
        }
        shortcuts
    }

    fn compute_super_shortcuts(&self, path: Vec<Position>, distances: HashMap<Position, u32>) -> Vec<u32> {
        let mut shortcuts = vec![];
        for position in path {
            let distance = distances.get(&position).unwrap();
            
            for candidate_position in self.reachable_positions_with_shortcut(position, 20) {
                if let Some(other_distance) = distances.get(&candidate_position) {
                    let distance_to_other = position.distance(&candidate_position);
                    if (distance + distance_to_other as u32) < *other_distance {
                        shortcuts.push(other_distance - distance - distance_to_other as u32);
                    }
                }
            }
        }
        shortcuts
    }

    fn reachable_positions_with_shortcut(&self, position: Position, length: usize) -> Vec<Position> {
        let mut positions = vec![];
        for x in 0..self.width {
            for y in 0..self.height {
                let candidate_position = Position { x, y };
                if position.distance(&candidate_position) <= length {
                    positions.push(candidate_position);
                }
            }
        }
        positions
    }

    fn find_first_move(&self) -> Position {
        let up = self.start.up();
        let down = self.start.down();
        let left = self.start.left();
        let right = self.start.right();

        if self.cell_at(up).is_on_path() {
            up
        } else if self.cell_at(down).is_on_path() {
            down
        } else if self.cell_at(left).is_on_path() {
            left
        } else {
            right
        }
    }

    fn find_next_move(&self, current_position: Position, previous_position: Position) -> Position {
        let up = current_position.up();
        let down = current_position.down();
        let left = current_position.left();
        let right = current_position.right();

        if self.cell_at(up).is_on_path() && up != previous_position {
            up
        } else if self.cell_at(down).is_on_path() && down != previous_position {
            down
        } else if self.cell_at(left).is_on_path() && left != previous_position {
            left
        } else {
            right
        }
    }
}

fn main() {
    let input = include_str!("../../input/day-20");
    let cpu = Cpu::from(input);
    let (path, distances) = cpu.compute_path();
    let shortcuts = cpu.compute_shortcuts(path.clone(), distances.clone());
    let shortcuts_greater_than_100 = shortcuts.values().filter(|value| **value >= 100).count();
    println!("There are {} shortcuts saving at least 100 picoseconds", shortcuts_greater_than_100);

    let super_shortcats = cpu.compute_super_shortcuts(path, distances);
    let super_shortcuts_greater_than_100 = super_shortcats.iter().filter(|value| **value >= 100).count();
    println!("There are {} super shortcuts saving at least 100 picoseconds", super_shortcuts_greater_than_100);
}

#[cfg(test)]
mod tests {
    use crate::Cpu;

    #[test]
    fn test_part1_shortest_path() {
        let input = include_str!("../../input/day-20-test");
        let cpu = Cpu::from(input);
        let (_, shortest_path) = cpu.compute_path();
        assert_eq!(shortest_path.get(&cpu.end), Some(&84));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day-20-test");
        let cpu = Cpu::from(input);
        let (path, distances) = cpu.compute_path();
        let shortcuts = cpu.compute_shortcuts(path, distances);
        let shortcut_values: Vec<_> = shortcuts.values().collect();
        assert_eq!(shortcut_values.iter().filter(|&&d| *d == 2).count(), 14);
        assert_eq!(shortcut_values.iter().filter(|&&d| *d == 4).count(), 14);
        assert_eq!(shortcut_values.iter().filter(|&&d| *d == 6).count(), 2);
        assert_eq!(shortcut_values.iter().filter(|&&d| *d == 8).count(), 4);
        assert_eq!(shortcut_values.iter().filter(|&&d| *d == 10).count(), 2);
        assert_eq!(shortcut_values.iter().filter(|&&d| *d == 12).count(), 3);
        assert_eq!(shortcut_values.iter().filter(|&&d| *d == 20).count(), 1);
        assert_eq!(shortcut_values.iter().filter(|&&d| *d == 36).count(), 1);
        assert_eq!(shortcut_values.iter().filter(|&&d| *d == 38).count(), 1);
        assert_eq!(shortcut_values.iter().filter(|&&d| *d == 40).count(), 1);
        assert_eq!(shortcut_values.iter().filter(|&&d| *d == 64).count(), 1);
    }
    
    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day-20-test");
        let cpu = Cpu::from(input);
        let (path, distances) = cpu.compute_path();
        let super_shortcut = cpu.compute_super_shortcuts(path, distances);
        assert_eq!(super_shortcut.iter().filter(|&&value| value == 50).count(), 32);
        assert_eq!(super_shortcut.iter().filter(|&&value| value == 52).count(), 31);
        assert_eq!(super_shortcut.iter().filter(|&&value| value == 54).count(), 29);
        assert_eq!(super_shortcut.iter().filter(|&&value| value == 56).count(), 39);
        assert_eq!(super_shortcut.iter().filter(|&&value| value == 58).count(), 25);
        assert_eq!(super_shortcut.iter().filter(|&&value| value == 60).count(), 23);
        assert_eq!(super_shortcut.iter().filter(|&&value| value == 62).count(), 20);
        assert_eq!(super_shortcut.iter().filter(|&&value| value == 64).count(), 19);
        assert_eq!(super_shortcut.iter().filter(|&&value| value == 66).count(), 12);
        assert_eq!(super_shortcut.iter().filter(|&&value| value == 68).count(), 14);
        assert_eq!(super_shortcut.iter().filter(|&&value| value == 70).count(), 12);
        assert_eq!(super_shortcut.iter().filter(|&&value| value == 72).count(), 22);
        assert_eq!(super_shortcut.iter().filter(|&&value| value == 74).count(), 4);
        assert_eq!(super_shortcut.iter().filter(|&&value| value == 76).count(), 3);
    }
}