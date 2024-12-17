use std::collections::HashMap;

#[derive(PartialEq)]
enum Cell {
    Wall,
    Empty,
    Start,
    End,
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
    North,
    South,
    West,
    East,
}

impl Direction {
    fn rotate_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::East => Direction::North,
        }
    }

    fn rotate_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::East => Direction::South,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn move_forward(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => Position { x: self.x, y: self.y-1 },
            Direction::South => Position { x: self.x, y: self.y+1 },
            Direction::West => Position { x: self.x-1, y: self.y },
            Direction::East => Position { x: self.x+1, y: self.y },
        }
    }
}

struct Maze {
    cells: Vec<Vec<Cell>>
}

impl Maze {
    fn compute_lowest_score_and_tiles_number(&self) -> (u32, usize) {
        let start = Position { x: 1, y: self.cells.len() - 2 };
        let end = Position { x: self.cells[0].len() - 2, y: 1 };
        let mut scores: HashMap<(Position, Direction), (u32, Vec<Position>)> = HashMap::new();

        self.visit_maze_rec(start, Direction::East, 0, vec![], &mut scores);

        let (score, positions) = [Direction::East, Direction::North, Direction::South, Direction::West].iter().filter_map(|d| 
            scores.get(&(end, *d))
        ).min_by_key(|(score, _)| score).unwrap();
        let mut p = positions.clone();
        p.sort();
        p.dedup();
        (*score, p.len())
    }

    fn visit_maze_rec(&self, position: Position, direction: Direction, score: u32, path: Vec<Position>, scores: &mut HashMap<(Position, Direction), (u32, Vec<Position>)>) {
        let best_current_score = self.get_current_best_score(scores);

        match self.cells[position.y][position.x] {
            Cell::Wall => return,
            Cell::Empty | Cell::Start => {
                if let Some((best_score, positions)) = scores.get(&(position, direction)) {
                    if let Some(best_score) = best_current_score {
                        if best_score < score {
                            return;
                        }
                    }
                    if score < *best_score {
                        let mut new_path = path;
                        new_path.push(position);

                        scores.insert((position, direction), (score, new_path.clone()));

                        let new_position = position.move_forward(direction);
                        let direction_left = direction.rotate_left();
                        let new_position_left = position.move_forward(direction_left);
                        let direction_right = direction.rotate_right();
                        let new_position_right = position.move_forward(direction_right);
                        self.visit_maze_rec(new_position, direction, score+1, new_path.clone(), scores);
                        self.visit_maze_rec(new_position_left, direction_left, score+1001, new_path.clone(), scores);
                        self.visit_maze_rec(new_position_right, direction_right, score+1001, new_path, scores);
                    } else if score == *best_score {
                        let mut new_path = path;
                        new_path.push(position);
                        let mut new_positions = positions.clone();
                        new_positions.append(&mut new_path.clone());

                        scores.insert((position, direction), (score, new_positions));

                        let new_position = position.move_forward(direction);
                        let direction_left = direction.rotate_left();
                        let new_position_left = position.move_forward(direction_left);
                        let direction_right = direction.rotate_right();
                        let new_position_right = position.move_forward(direction_right);
                        self.visit_maze_rec(new_position, direction, score+1, new_path.clone(), scores);
                        self.visit_maze_rec(new_position_left, direction_left, score+1001, new_path.clone(), scores);
                        self.visit_maze_rec(new_position_right, direction_right, score+1001, new_path, scores);
                    }
                } else {
                    if let Some(best_score) = best_current_score {
                        if best_score < score {
                            return;
                        }
                    }
                    let mut new_path = path;
                    new_path.push(position);

                    scores.insert((position, direction), (score, new_path.clone()));

                    let new_position = position.move_forward(direction);
                    let direction_left = direction.rotate_left();
                    let new_position_left = position.move_forward(direction_left);
                    let direction_right = direction.rotate_right();
                    let new_position_right = position.move_forward(direction_right);
                    self.visit_maze_rec(new_position, direction, score+1, new_path.clone(), scores);
                    self.visit_maze_rec(new_position_left, direction_left, score+1001, new_path.clone(), scores);
                    self.visit_maze_rec(new_position_right, direction_right, score+1001, new_path, scores);
                }
            },
            Cell::End => {
                if let Some((best_score, positions)) = scores.get(&(position, direction)) {
                    if score < *best_score {
                        println!("New best score is: {}", score);
                        let mut new_path = path;
                        new_path.push(position);
                        scores.insert((position, direction), (score, new_path));
                    } else if score == *best_score {
                        println!("Best score of {} is equalled", score);
                        let mut new_path = path;
                        new_path.push(position);
                        let mut new_positions = positions.clone();
                        new_positions.append(&mut new_path);

                        scores.insert((position, direction), (score, new_positions));
                    }
                } else {
                    println!("First best score is: {}", score);
                    let mut new_path = path;
                    new_path.push(position);
                    scores.insert((position, direction), (score, new_path));
                }
                return;
            },
        }
    }

    fn get_current_best_score(&self, scores: &HashMap<(Position, Direction), (u32, Vec<Position>)>) -> Option<u32> {
        let end = Position { x: self.cells[0].len() - 2, y: 1 };
        let best = [Direction::East, Direction::North, Direction::South, Direction::West].iter().filter_map(|d| 
            scores.get(&(end, *d))
        ).min_by_key(|(score, _)| score);
        best.map(|best| best.0)
    }
}

impl From<&str> for Maze {
    fn from(input: &str) -> Self {
        let cells: Vec<Vec<Cell>> = input.lines()
            .map(|line| line.chars().map(Cell::from).collect()).collect();

        Self { cells }
    }
}

fn main() {
    let input = include_str!("../../input/day-16");
    let maze = Maze::from(input);
    let (score, number_of_tiles) = maze.compute_lowest_score_and_tiles_number();
    println!("Score: {}", score);
    println!("Number of tiles: {}", number_of_tiles);
}

#[cfg(test)]
mod tests {
    use crate::Maze;

    #[test]
    fn test_part1_small() {
        let input = include_str!("../../input/day-16-test-small");
        let maze = Maze::from(input);
        let (score, _) = maze.compute_lowest_score_and_tiles_number();
        assert_eq!(score, 7036);
    }
    
    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day-16-test");
        let maze = Maze::from(input);
        let (score, _) = maze.compute_lowest_score_and_tiles_number();
        assert_eq!(score, 11048);
    }
    
    #[test]
    fn test_part2_small() {
        let input = include_str!("../../input/day-16-test-small");
        let maze = Maze::from(input);
        let (_, tiles) = maze.compute_lowest_score_and_tiles_number();
        assert_eq!(tiles, 45);
    }
    
    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day-16-test");
        let maze = Maze::from(input);
        let (_, tiles) = maze.compute_lowest_score_and_tiles_number();
        assert_eq!(tiles, 64);
    }
}