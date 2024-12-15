#[derive(PartialEq)]
enum Cell {
    Wall,
    Box,
    Empty,
    Robot,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            'O' => Self::Box,
            '.' => Self::Empty,
            '@' => Self::Robot,
            _ => unreachable!(),
        }
    }
}

impl Cell {
    fn scale_up(&self) -> [BigCell;2] {
        match self {
            Cell::Wall => [BigCell::Wall, BigCell::Wall],
            Cell::Box => [BigCell::BoxL, BigCell::BoxR],
            Cell::Empty => [BigCell::Empty, BigCell::Empty],
            Cell::Robot => [BigCell::Robot, BigCell::Empty],
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum BigCell {
    Wall,
    BoxL,
    BoxR,
    Empty,
    Robot,
}

impl BigCell {
    fn is_box(&self) -> bool {
        match self {
            BigCell::BoxL | BigCell::BoxR => true,
            _ => false,
        }
    }
}

#[derive(Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(c: char) -> Self {
        match c {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => unreachable!(),
        }
    }
}

impl From<BigCell> for char {
    fn from(cell: BigCell) -> Self {
        match cell {
            BigCell::Wall => '#',
            BigCell::BoxL => '[',
            BigCell::BoxR => ']',
            BigCell::Empty => '.',
            BigCell::Robot => '@',
        }
    }
}

struct Warehouse {
    cells: Vec<Vec<Cell>>,
    robot: Position,
    instructions: Vec<Instruction>
}

impl Warehouse {
    fn follow_instructions(&mut self) {
        let instructions = self.instructions.clone();
        instructions.into_iter().for_each(|instruction| self.follow_instruction(instruction));
    }

    fn follow_instruction(&mut self, instruction: Instruction) {
        // the nice thing with having a full boundary of walls is that we don't have to check coordinates
        let x = self.robot.x;
        let y = self.robot.y;
        match instruction {
            Instruction::Up => {
                match self.cells[y-1][x] {
                    Cell::Wall => {/* nothing to do, robot is against a wall */},
                    Cell::Box => {
                        let mut forward_y = y-1;
                        while self.cells[forward_y][x] == Cell::Box {
                            forward_y -= 1;
                        }
                        if self.cells[forward_y][x] == Cell::Wall {
                            /* nothing to do, boxes are stacked against a wall */
                        } else {
                            // there's an empty space at forward_y
                            self.cells[forward_y][x] = Cell::Box;
                            self.cells[y][x] = Cell::Empty;
                            self.cells[y-1][x] = Cell::Robot;
                            self.robot = Position{x, y: y-1};
                        }
                    },
                    Cell::Empty => {
                        self.cells[y][x] = Cell::Empty;
                        self.cells[y-1][x] = Cell::Robot;
                        self.robot = Position{x, y: y-1};
                    },
                    Cell::Robot => unreachable!(),
                }
            },
            Instruction::Down => {
                match self.cells[y+1][x] {
                    Cell::Wall => {/* nothing to do, robot is against a wall */},
                    Cell::Box => {
                        let mut forward_y = y+1;
                        while self.cells[forward_y][x] == Cell::Box {
                            forward_y += 1;
                        }
                        if self.cells[forward_y][x] == Cell::Wall {
                            /* nothing to do, boxes are stacked against a wall */
                        } else {
                            // there's an empty space at forward_y
                            self.cells[forward_y][x] = Cell::Box;
                            self.cells[y][x] = Cell::Empty;
                            self.cells[y+1][x] = Cell::Robot;
                            self.robot = Position{x, y: y+1};
                        }
                    },
                    Cell::Empty => {
                        self.cells[y][x] = Cell::Empty;
                        self.cells[y+1][x] = Cell::Robot;
                        self.robot = Position{x, y: y+1};
                    },
                    Cell::Robot => unreachable!(),
                }
            },
            Instruction::Left => {
                match self.cells[y][x-1] {
                    Cell::Wall => {/* nothing to do, robot is against a wall */},
                    Cell::Box => {
                        let mut forward_x = x-1;
                        while self.cells[y][forward_x] == Cell::Box {
                            forward_x -= 1;
                        }
                        if self.cells[y][forward_x] == Cell::Wall {
                            /* nothing to do, boxes are stacked against a wall */
                        } else {
                            // there's an empty space at forward_y
                            self.cells[y][forward_x] = Cell::Box;
                            self.cells[y][x] = Cell::Empty;
                            self.cells[y][x-1] = Cell::Robot;
                            self.robot = Position{x: x-1, y};
                        }
                    },
                    Cell::Empty => {
                        self.cells[y][x] = Cell::Empty;
                        self.cells[y][x-1] = Cell::Robot;
                        self.robot = Position{x: x-1, y};
                    },
                    Cell::Robot => unreachable!(),
                }
            },
            Instruction::Right => {
                match self.cells[y][x+1] {
                    Cell::Wall => {/* nothing to do, robot is against a wall */},
                    Cell::Box => {
                        let mut forward_x = x+1;
                        while self.cells[y][forward_x] == Cell::Box {
                            forward_x += 1;
                        }
                        if self.cells[y][forward_x] == Cell::Wall {
                            /* nothing to do, boxes are stacked against a wall */
                        } else {
                            // there's an empty space at forward_y
                            self.cells[y][forward_x] = Cell::Box;
                            self.cells[y][x] = Cell::Empty;
                            self.cells[y][x+1] = Cell::Robot;
                            self.robot = Position{x: x+1, y};
                        }
                    },
                    Cell::Empty => {
                        self.cells[y][x] = Cell::Empty;
                        self.cells[y][x+1] = Cell::Robot;
                        self.robot = Position{x: x+1, y};
                    },
                    Cell::Robot => unreachable!(),
                }
            },
        }
    }

    fn sum_boxes_coordinates(&self) -> usize {
        self.cells
            .iter()
            .enumerate()
            .map(|(y, line)| 
                line.iter().enumerate().filter(|(_, cell)| **cell == Cell::Box).map(|(x,_)| 100*y + x).sum::<usize>()
            ).sum()
    }

    fn scale_up(&self) -> BigWarehouse {
        let instructions = self.instructions.clone();
        let cells: Vec<Vec<BigCell>> = self.cells.iter()
            .map(|line|
                line.iter().map(|cell| cell.scale_up()).flatten().collect()
            ).collect();
        let robot = cells.iter().enumerate().filter_map(|(y, line)| line.iter().position(|c| *c == BigCell::Robot).map(|x| Position{x, y})).next().unwrap();

        BigWarehouse { cells, robot, instructions }
    }
}

impl From<&str> for Warehouse {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();
        let cells: Vec<Vec<Cell>> = lines.by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| line.chars().map(Cell::from).collect()).collect();
        let robot = cells.iter().enumerate().filter_map(|(y, line)| line.iter().position(|c| *c == Cell::Robot).map(|x| Position{x, y})).next().unwrap();

        let instructions = lines.map(|line| line.chars().map(Instruction::from)).flatten().collect();

        Self { cells, robot, instructions }
    }
}

struct BigWarehouse {
    cells: Vec<Vec<BigCell>>,
    robot: Position,
    instructions: Vec<Instruction>
}

impl BigWarehouse {
    fn follow_instructions(&mut self) {
        let instructions = self.instructions.clone();
        instructions.into_iter().for_each(|instruction| {
            self.follow_instruction(instruction);
            //self.draw();
        });
    }

    fn draw(&self) {
        println!("Robot in {}, {}", self.robot.x, self.robot.y);
        self.cells.iter().for_each(|line| {
            let mut to_print = "".to_string();
            line.iter().for_each(|&cell| to_print.push(cell.into()));
            println!("{to_print}");
        });
        println!();
    }

    fn follow_instruction(&mut self, instruction: Instruction) {
        // the nice thing with having a full boundary of walls is that we don't have to check coordinates
        let x = self.robot.x;
        let y = self.robot.y;
        match instruction {
            Instruction::Up => {
                match self.cells[y-1][x] {
                    BigCell::Wall => { /* nothing to do */},
                    BigCell::BoxL => {
                        if self.can_push_box_up_down(x, y-1, true) {
                            self.push_box_up_down(x, y-1, true);
                            self.move_robot(x, y-1);
                        }
                    },
                    BigCell::BoxR => {
                        if self.can_push_box_up_down(x-1, y-1, true) {
                            self.push_box_up_down(x-1, y-1, true);
                            self.move_robot(x, y-1);
                        }
                    }
                    BigCell::Empty => {
                        self.move_robot(x, y-1);
                    },
                    BigCell::Robot => unreachable!(),
                }
            },
            Instruction::Down => {
                match self.cells[y+1][x] {
                    BigCell::Wall => { /* nothing to do */},
                    BigCell::BoxL => {
                        if self.can_push_box_up_down(x, y+1, false) {
                            self.push_box_up_down(x, y+1, false);
                            self.move_robot(x, y+1);
                        }
                    },
                    BigCell::BoxR => {
                        if self.can_push_box_up_down(x-1, y+1, false) {
                            self.push_box_up_down(x-1, y+1, false);
                            self.move_robot(x, y+1);
                        }
                    }
                    BigCell::Empty => {
                        self.move_robot(x, y+1);
                    },
                    BigCell::Robot => unreachable!(),
                }
            },
            Instruction::Left => {
                match self.cells[y][x-1] {
                    BigCell::Wall => { /* nothing to do */},
                    BigCell::BoxR => {
                        if self.can_push_box_left_right(x-1, y, true) {
                            self.push_box_left_right(x-1, y, true);
                            self.move_robot(x-1, y);
                        }
                    },
                    BigCell::Empty => {
                        self.move_robot(x-1, y);
                    },
                    BigCell::Robot | BigCell::BoxL => unreachable!(),
                }
            },
            Instruction::Right => {
                match self.cells[y][x+1] {
                    BigCell::Wall => { /* nothing to do */},
                    BigCell::BoxL => {
                        if self.can_push_box_left_right(x+1, y, false) {
                            self.push_box_left_right(x+1, y, false);
                            self.move_robot(x+1, y);
                        }
                    },
                    BigCell::Empty => {
                        self.move_robot(x+1, y);
                    },
                    BigCell::Robot | BigCell::BoxR => unreachable!(),
                }
            },
        }
    }

    fn move_robot(&mut self, new_x: usize, new_y: usize) {
        self.cells[self.robot.y][self.robot.x] = BigCell::Empty;
        self.cells[new_y][new_x] = BigCell::Robot;
        self.robot = Position{x: new_x, y: new_y};
    }

    fn can_push_box_up_down(&self, x: usize, y: usize, go_up: bool) -> bool {
        let new_y = if go_up { y - 1 } else { y + 1 };
        match (self.cells[new_y][x], self.cells[new_y][x+1]) {
            (BigCell::Wall, _) | (_, BigCell::Wall) => false,
            (BigCell::Empty, BigCell::Empty) => true,
            (BigCell::BoxL, BigCell::BoxR) => self.can_push_box_up_down(x, new_y, go_up),
            (BigCell::BoxR, BigCell::BoxL) => self.can_push_box_up_down(x-1, new_y, go_up) && self.can_push_box_up_down(x+1, new_y, go_up),
            (BigCell::BoxR, BigCell::Empty) => self.can_push_box_up_down(x-1, new_y, go_up),
            (BigCell::Empty, BigCell::BoxL) => self.can_push_box_up_down(x+1, new_y, go_up),
            _ => {
                println!("{:?} {:?}", self.cells[new_y][x], self.cells[new_y][x+1]);
                unreachable!()
            },
        }
    }

    fn push_box_up_down(&mut self, x: usize, y: usize, go_up: bool) {
        let new_y = if go_up { y - 1 } else { y + 1 };
        match (self.cells[new_y][x], self.cells[new_y][x+1]) {
            (BigCell::Empty, BigCell::Empty) => {
                self.move_box_up_down(x, y, go_up);
            },
            (BigCell::BoxL, BigCell::BoxR) => {
                self.push_box_up_down(x, new_y, go_up);
                self.move_box_up_down(x, y, go_up);
            },
            (BigCell::BoxR, BigCell::BoxL) => {
                self.push_box_up_down(x-1, new_y, go_up);
                self.push_box_up_down(x+1, new_y, go_up);
                self.move_box_up_down(x, y, go_up);
            },
            (BigCell::BoxR, BigCell::Empty) => {
                self.push_box_up_down(x-1, new_y, go_up);
                self.move_box_up_down(x, y, go_up);
            },
            (BigCell::Empty, BigCell::BoxL) => {
                self.push_box_up_down(x+1, new_y, go_up);
                self.move_box_up_down(x, y, go_up);
            },
            _ => unreachable!(),
        }
    }

    fn move_box_up_down(&mut self, x: usize, y: usize, go_up: bool) {
        let new_y = if go_up { y - 1 } else { y + 1 };
        self.cells[new_y][x] = BigCell::BoxL;
        self.cells[new_y][x+1] = BigCell::BoxR;
        self.cells[y][x] = BigCell::Empty;
        self.cells[y][x+1] = BigCell::Empty;
    }

    fn can_push_box_left_right(&self, x: usize, y: usize, go_left: bool) -> bool {
        let new_x = if go_left { x - 1 } else { x + 1 };
        match self.cells[y][new_x] {
            BigCell::Wall => false,
            BigCell::BoxL | BigCell::BoxR => self.can_push_box_left_right(new_x, y, go_left),
            BigCell::Empty => true,
            BigCell::Robot => unreachable!(),
        }
    }

    fn push_box_left_right(&mut self, x: usize, y: usize, go_left: bool) {
        let new_x = if go_left { x - 1 } else { x + 1 };
        match self.cells[y][new_x] {
            BigCell::BoxL | BigCell::BoxR => {
                self.push_box_left_right(new_x, y, go_left);
                self.move_box_left_right(x, y, go_left);
            },
            BigCell::Empty => {
                self.move_box_left_right(x, y, go_left);
            },
            _ => unreachable!(),
        }
    }

    fn move_box_left_right(&mut self, x: usize, y: usize, go_left: bool) {
        let new_x = if go_left { x - 1 } else { x + 1 };
        self.cells[y][new_x] = self.cells[y][x];
        self.cells[y][x] = BigCell::Empty;
    }

    fn sum_boxes_coordinates(&self) -> usize {
        self.cells
            .iter()
            .enumerate()
            .map(|(y, line)| 
                line.iter().enumerate().filter(|(_, cell)| **cell == BigCell::BoxL).map(|(x,_)| 100*y + x).sum::<usize>()
            ).sum()
    }
}

fn main() {
    let input = include_str!("../../input/day-15");
    let mut warehouse = Warehouse::from(input);
    let mut big_warehouse = warehouse.scale_up();
    warehouse.follow_instructions();
    let sum = warehouse.sum_boxes_coordinates();
    println!("Sum of coordinates after instructions: {}", sum);

    big_warehouse.follow_instructions();
    let sum_big = big_warehouse.sum_boxes_coordinates();
    println!("Sum of coordinates after instructions for big warehouge: {}", sum_big);
}

#[cfg(test)]
mod tests {
    use crate::Warehouse;

    #[test]
    fn test_part1_small() {
        let input = include_str!("../../input/day-15-test-small");
        let mut warehouse = Warehouse::from(input);
        warehouse.follow_instructions();
        let sum = warehouse.sum_boxes_coordinates();
        assert_eq!(sum, 2028);
    }
    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day-15-test");
        let mut warehouse = Warehouse::from(input);
        warehouse.follow_instructions();
        let sum = warehouse.sum_boxes_coordinates();
        assert_eq!(sum, 10092);
    }
    
    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day-15-test");
        let warehouse = Warehouse::from(input);
        let mut big_warehouse = warehouse.scale_up();
        big_warehouse.draw();
        big_warehouse.follow_instructions();
        let sum = big_warehouse.sum_boxes_coordinates();
        assert_eq!(sum, 9021);
    }
}