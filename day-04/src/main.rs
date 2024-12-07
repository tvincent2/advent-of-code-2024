struct XmasLines {
    lines: Vec<String>,
}

impl XmasLines {
    fn new(input: &str) -> Self {
        let lines = input.lines().map(|line| line.to_string()).collect();
        XmasLines{ lines }
    }

    fn count_xmas(&self) -> usize {
        self.count_horizontal() + self.count_vertical() + self.count_diagonal1() + self.count_diagonal2()
    }

    fn count_horizontal(&self) -> usize {
        let count_forward = self.lines.iter().map(|line| line.matches("XMAS").count()).sum::<usize>();
        let count_backward = self.lines.iter().map(|line| line.matches("SAMX").count()).sum::<usize>();
        count_backward + count_forward
    }

    fn count_vertical(&self) -> usize {
        let vert = self.make_vertical();
        vert.count_horizontal()
    }

    fn count_diagonal1(&self) -> usize {
        let diag = self.make_diagonal1();
        diag.count_horizontal()
    }

    fn count_diagonal2(&self) -> usize {
        let diag = self.make_diagonal2();
        diag.count_horizontal()
    }

    fn make_vertical(&self) -> XmasLines {
        let mut lines = vec!["".to_string(); self.lines[0].len()];
        self.lines.iter().for_each(|line|
            line.char_indices().for_each(|(index, c)| lines[index].push(c))
        );
        XmasLines{ lines }
    }

    fn make_diagonal1(&self) -> XmasLines {
        let mut lines = vec!["".to_string(); self.lines.len() + self.lines[0].len()];
        self.lines.iter().enumerate().for_each(|(line_index, line)|
            line.char_indices().for_each(|(char_index, c)| lines[line_index + char_index].push(c))
        );
        XmasLines{ lines }
    }

    fn make_diagonal2(&self) -> XmasLines {
        let mut lines = vec!["".to_string(); self.lines.len() + self.lines[0].len()];
        self.lines.iter().enumerate().for_each(|(line_index, line)|
            line.chars().rev().enumerate().for_each(|(char_index, c)| lines[line_index + char_index].push(c))
        );
        XmasLines{ lines }
    }

    fn count_x_mas(&self) -> usize {
        let total_lines = self.lines.len();
        let total_columns = self.lines[0].len();
        self.lines.iter().enumerate().map(|(line_index, line)| 
            line.char_indices().filter(|(char_index, c)| 
                if *c == 'A' && line_index > 0 && *char_index > 0 && line_index < total_lines - 1 && *char_index < total_columns - 1 {
                    self.is_x_mas(line_index, *char_index)
                } else {
                    false
                }
            ).count()
        ).sum()
    }

    fn is_x_mas(&self, line_index: usize, char_index: usize) -> bool {
        let north_west = self.lines[line_index - 1].chars().nth(char_index - 1).unwrap();
        let south_east = self.lines[line_index + 1].chars().nth(char_index + 1).unwrap();
        let is_diag1_ok = (north_west == 'S' && south_east == 'M') || (north_west == 'M' && south_east == 'S');

        let north_east = self.lines[line_index - 1].chars().nth(char_index + 1).unwrap();
        let south_west = self.lines[line_index + 1].chars().nth(char_index - 1).unwrap();
        let is_diag2_ok = (north_east == 'S' && south_west == 'M') || (north_east == 'M' && south_west == 'S');

        is_diag1_ok && is_diag2_ok
    }
}

fn main() {
    let input = include_str!("../../input/day-04");
    let lines = XmasLines::new(input);
    let count = lines.count_xmas();
    println!("Number of XMAS occurrences: {count}");

    let count_part_2 = lines.count_x_mas();
    println!("Number of X-MAS occurrences: {count_part_2}");
}

#[cfg(test)]
mod tests {
    use crate::XmasLines;


    #[test]
    fn test_part1_horizontal() {
        let input = include_str!("../../input/day-04-test");
        let lines = XmasLines::new(input);
        let count_horizontal = lines.count_horizontal();
        assert_eq!(count_horizontal, 5);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day-04-test");
        let lines = XmasLines::new(input);
        let count = lines.count_xmas();
        assert_eq!(count, 18);
    }
    
    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day-04-test");
        let lines = XmasLines::new(input);
        let count = lines.count_x_mas();
        assert_eq!(count, 9);
    }
}