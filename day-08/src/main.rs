#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

struct Antenna {
    position: Position,
    frequency: char,
}

struct AntennaMap {
    width: i32,
    height: i32,
    antennas: Vec<Antenna>
}

impl AntennaMap {
    fn count_antinodes(&self) -> usize {
        let mut range: Vec<char> = ('0'..='9').collect();
        range.extend('a'..='z');
        range.extend('A'..='Z');

        let mut antinodes = range.iter().map(|c| self.antinodes_for_frequency(*c)).flatten().filter(|antinode| self.is_valid_antinode(antinode)).collect::<Vec<_>>();
        antinodes.sort();
        antinodes.dedup();
        antinodes.len()
    }

    fn antinodes_for_frequency(&self, frequency: char) -> Vec<Position> {
        let antennas = self.antennas.iter().filter(|antenna| antenna.frequency == frequency).collect::<Vec<&Antenna>>();
        let mut antinodes = vec![];
        for a in 0..antennas.len() {
            for b in (a+1)..antennas.len() {
                let position_a = antennas[a].position;
                let position_b = antennas[b].position;
                let diff_x = position_a.x - position_b.x;
                let diff_y = position_a.y - position_b.y;

                let antinode_a = Position{x: position_a.x + diff_x, y: position_a.y + diff_y};
                antinodes.push(antinode_a);
                let antinode_b = Position{x: position_b.x - diff_x, y: position_b.y - diff_y};
                antinodes.push(antinode_b);

            }
        }
        antinodes
    }

    fn count_antinodes_with_harmonics(&self) -> usize {
        let mut range: Vec<char> = ('0'..='9').collect();
        range.extend('a'..='z');
        range.extend('A'..='Z');

        let mut antinodes = range.iter().map(|c| self.antinodes_for_frequency_with_harmonics(*c)).flatten().collect::<Vec<_>>();
        antinodes.sort();
        antinodes.dedup();
        antinodes.len()
    }

    fn antinodes_for_frequency_with_harmonics(&self, frequency: char) -> Vec<Position> {
        let antennas = self.antennas.iter().filter(|antenna| antenna.frequency == frequency).collect::<Vec<&Antenna>>();
        let mut antinodes = vec![];
        for a in 0..antennas.len() {
            for b in (a+1)..antennas.len() {
                let position_a = antennas[a].position;
                let position_b = antennas[b].position;
                let diff_x = position_a.x - position_b.x;
                let diff_y = position_a.y - position_b.y;

                let mut antinode_a = position_a;
                while self.is_valid_antinode(&antinode_a) {
                    antinodes.push(antinode_a);
                    antinode_a = Position{x: antinode_a.x + diff_x, y: antinode_a.y + diff_y};
                }

                let mut antinode_b = position_b;
                while self.is_valid_antinode(&antinode_b) {
                    antinodes.push(antinode_b);
                    antinode_b = Position{x: antinode_b.x - diff_x, y: antinode_b.y - diff_y};
                }
            }
        }
        antinodes
    }

    fn is_valid_antinode(&self, position: &Position) -> bool {
        position.x >= 0 && position.x < self.width && position.y >= 0 && position.y < self.height
    }
}

impl From<&str> for AntennaMap {
    fn from(input: &str) -> Self {
        let height = input.lines().fold(0, |acc, _| acc + 1);
        let width = input.lines().next().unwrap().len() as i32;
        let cells = input.lines().enumerate().map(|(y, line)| {
            line.char_indices().filter(|(_, c)| c != &'.').map(|(x, c)| Antenna{position:Position{x: x as i32, y: y as i32}, frequency: c}).collect::<Vec<Antenna>>()
        }).flatten().collect();
        Self{ height, width, antennas: cells }
    }
}

fn main() {
    let input = include_str!("../../input/day-08");
    let antenna_map = AntennaMap::from(input);
    let number_of_antinodes = antenna_map.count_antinodes();
    println!("Number of antinodes: {}", number_of_antinodes);

    let number_of_antinodes_with_harmonics = antenna_map.count_antinodes_with_harmonics();
    println!("Number of antinodes with harmonics: {}", number_of_antinodes_with_harmonics);
}

#[cfg(test)]
mod tests {
    use crate::AntennaMap;


    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day-08-test");
        let antenna_map = AntennaMap::from(input);
        let count = antenna_map.count_antinodes();
        assert_eq!(count, 14);
    }
    
    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day-08-test");
        let antenna_map = AntennaMap::from(input);
        let count = antenna_map.count_antinodes_with_harmonics();
        assert_eq!(count, 34);
    }
}