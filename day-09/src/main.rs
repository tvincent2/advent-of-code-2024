#[derive(Debug,Clone, Copy, PartialEq)]
enum State {
    Empty,
    Full(u64),
}

impl State {
    fn unwrap_or_zero(&self) -> u64 {
        match self {
            State::Empty => 0,
            State::Full(value) => *value,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            State::Empty => true,
            State::Full(_) => false,
        }
    }
}

struct FileSystem {
    layout: Vec<State>
}

impl FileSystem {
    fn condense(&self) -> Self {
        let mut layout = vec![];
        let mut rev_full_iter = self.layout.iter().enumerate().filter_map(|(index, &cell)| match cell {
            State::Empty => None,
            State::Full(value) => Some((index, value)),
        }).rev();
        let mut current_end_index = self.layout.len();
        for (index, cell) in self.layout.iter().enumerate() {
            match cell {
                State::Empty => {
                    let (index_from_end, value_from_end) = rev_full_iter.next().unwrap();
                    current_end_index = index_from_end;
                    if index > current_end_index {
                        break;
                    } else {
                        layout.push(State::Full(value_from_end));
                        current_end_index -= 1;
                    }
                },
                State::Full(value) => {
                    if index > current_end_index {
                        break;
                    } else {
                        layout.push(State::Full(*value))
                    }
                },
            }
        }
        Self { layout }
    }

    fn condense_without_fragmentation(&self) -> Self {
        let mut file_location = self.layout.len() - 1;
        let mut file_size = 0;
        let mut layout = self.layout.clone();
        while file_location > 1 {
            (file_location, file_size) = find_next_file(&layout, file_location);
            if let Some(new_location) = find_better_location(&layout, file_location, file_size) {
                for offset in 0..file_size {
                    layout[new_location + offset] = layout[file_location + offset];
                    layout[file_location + offset] = State::Empty;
                }
            }
            file_location -= 1;
        }
        Self { layout }
    }


    fn compute_checksum(&self) -> u64 {
        self.layout.iter().enumerate().map(|(index, value)| index as u64 * value.unwrap_or_zero()).sum()
    }
}

fn find_next_file(layout: &Vec<State>, current_location: usize) -> (usize, usize) {
    let mut location = current_location;
    while layout[location].is_empty() {
        // no risk of underflow since the file system always starts with a file
        location -= 1;
    }
    let current_file = layout[location];
    let end_location = location;
    while location > 0 && layout[location - 1] == current_file {
        location -= 1;
    }
    (location, end_location - location + 1)
}

fn find_better_location(layout: &Vec<State>, file_location: usize, file_size: usize) -> Option<usize> {
    let mut location = 0;
    let mut size = 0;
    (location, size) = next_empty_location(layout, location);
    while location < file_location {
        if size < file_size {
            (location, size) = next_empty_location(layout, location + 1)
        } else {
            return Some(location);
        }
    }
    None
}

fn next_empty_location(layout: &Vec<State>, current_location: usize) -> (usize, usize) {
    let mut location = current_location;
    while !layout[location].is_empty() {
        location += 1;
    }
    let start_location = location;
    while location < layout.len() - 1 && layout[location + 1].is_empty() {
        location += 1;
    }
    (start_location, location - start_location + 1)
}

impl From<&str> for FileSystem {
    fn from(input: &str) -> Self {
        let mut index = 0;
        let mut full = true;
        let layout = input.chars().flat_map(|c| {
            let size = c.to_digit(10).unwrap() as usize;
            let space = if full { let state = State::Full(index); index += 1; state } else { State::Empty };
            full = !full;
            vec![space; size]
        }).collect();
        Self { layout }
    }
}

fn main() {
    let input = include_str!("../../input/day-09");
    let sparse_file_system = FileSystem::from(input);
    let condensed_file_system = sparse_file_system.condense();
    let checksum = condensed_file_system.compute_checksum();
    println!("Checksum: {}", checksum);

    let condensed_file_system_without_fragmentation = sparse_file_system.condense_without_fragmentation();
    let checksum_without_fragmentation = condensed_file_system_without_fragmentation.compute_checksum();
    println!("Checksum: {}", checksum_without_fragmentation);
}

#[cfg(test)]
mod tests {
    use crate::FileSystem;


    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day-09-test");
        let sparse_file_system = FileSystem::from(input);
        let condensed_file_system = sparse_file_system.condense();
        let count = condensed_file_system.compute_checksum();
        assert_eq!(count, 1928);
    }
    
    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day-09-test");
        let sparse_file_system = FileSystem::from(input);
        let condensed_file_system = sparse_file_system.condense_without_fragmentation();
        let count = condensed_file_system.compute_checksum();
        assert_eq!(count, 2858);
    }
}