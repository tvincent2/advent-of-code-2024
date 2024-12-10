struct TopographicMap {
    map: Vec<Vec<u8>>
}

impl TopographicMap {
    fn count_trailheads_scores(&self) -> (usize, usize) {
        let height = self.map.len();
        let width = self.map[0].len();
        let mut map_with_count = vec![vec![vec![]; width]; height];
        let mut id = 0;
        for altitude in (0u8..=9u8).rev() {
            for y in 0..height {
                for x in 0..width {
                    if self.map[y][x] == altitude {
                        if altitude == 9 {
                            map_with_count[y][x].push(id);
                            id += 1;
                        } else {
                            let mut reachable = vec![];
                            // up
                            if y > 0 && self.map[y-1][x] == (altitude + 1) {
                                reachable.extend(map_with_count[y-1][x].iter());
                            }
                            // down
                            if y + 1 < height && self.map[y+1][x] == (altitude + 1) {
                                reachable.extend(map_with_count[y+1][x].iter());
                            }
                            // left
                            if x > 0 && self.map[y][x-1] == (altitude + 1) {
                                reachable.extend(map_with_count[y][x-1].iter());
                            }
                            // right
                            if x + 1 < width && self.map[y][x+1] == (altitude + 1) {
                                reachable.extend(map_with_count[y][x+1].iter());
                            }
                            map_with_count[y][x] = reachable;
                        }
                    }
                }
            }
        }

        let part_2 = self.map.iter().enumerate().map(|(y, line)| 
            line.iter().enumerate().filter(|(_, &altitude)| altitude == 0).map(|(x, _)| {
                map_with_count[y][x].len()
            }).sum::<usize>()
        ).sum();

        // println!("{:?}", map_with_count);
        let part_1 = self.map.iter().enumerate().map(|(y, line)| 
            line.iter().enumerate().filter(|(_, &altitude)| altitude == 0).map(|(x, _)| {
                map_with_count[y][x].sort();
                map_with_count[y][x].dedup();
                map_with_count[y][x].len()
            }).sum::<usize>()
        ).sum();
        (part_1, part_2)
    }
}

impl From<&str> for TopographicMap {
    fn from(input: &str) -> Self {
        let map = input.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()).collect();
        Self { map }
    }
}

fn main() {
    let input = include_str!("../../input/day-10");
    let topographic_map = TopographicMap::from(input);
    let (trailheads_score_1, trailheads_score_2) = topographic_map.count_trailheads_scores();
    println!("Score 1: {}", trailheads_score_1);
    println!("Score 1: {}", trailheads_score_2);
}

#[cfg(test)]
mod tests {
    use crate::TopographicMap;


    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day-10-test");
        let topographic_map = TopographicMap::from(input);
        let (count, _) = topographic_map.count_trailheads_scores();
        assert_eq!(count, 36);
    }
    
    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day-10-test");
        let topographic_map = TopographicMap::from(input);
        let (_, count) = topographic_map.count_trailheads_scores();
        assert_eq!(count, 81);
    }
}