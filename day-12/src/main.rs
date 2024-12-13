struct Garden {
    plants: Vec<Vec<char>>,
}

impl Garden {
    fn width(&self) -> usize {
        self.plants[0].len()
    }

    fn height(&self) -> usize {
        self.plants.len()
    }

    fn compute_regular_fencing_price(&self) -> u32 {
        self.compute_fencing_price(false)
    }

    fn compute_fencing_price_with_discount(&self) -> u32 {
        self.compute_fencing_price(true)
    }

    fn compute_fencing_price(&self, discount: bool) -> u32 {
        let mut price = 0;
        let width = self.plants[0].len();
        let height = self.plants.len();
        let mut plants_to_visit: Vec<Vec<(usize, usize, bool)>> = vec![];
        for y in 0..height {
            plants_to_visit.push((0..width).map(|x| (x, y, true)).collect());
        }
        while let Some((x_to_visit, y_to_visit, _)) = plants_to_visit.iter().flatten().find(|(_, _, to_visit)| *to_visit) {
            price += self.compute_fence_for_area(*x_to_visit, *y_to_visit, &mut plants_to_visit, discount);
        }
        price
    }

    fn compute_fence_for_area(&self, start_x: usize, start_y: usize, non_visited: &mut Vec<Vec<(usize, usize, bool)>>, discount: bool) -> u32 {
        let mut area = 0;
        let mut perimeter = 0;
        let mut plants_to_count = vec![(start_x, start_y)];
        while let Some((x, y)) = plants_to_count.pop() {
            if non_visited[y][x].2 {
                area += 1;
                let (local_perimeter, mut plants) = self.analyze_plant(x, y, non_visited, discount);
                perimeter += local_perimeter;
                plants_to_count.append(&mut plants);
                non_visited[y][x].2 = false;
            }
        }
        area * perimeter
    }

    fn analyze_plant(&self, x: usize, y: usize, non_visited: &mut Vec<Vec<(usize, usize, bool)>>, discount: bool) -> (u32, Vec<(usize, usize)>) {
        // guard against already visited plants
        let plant_type = self.plants[y][x];
        let mut neighbors = vec![];
        let mut perimeter = 0;
        // left
        if x > 0 && self.plants[y][x-1] == plant_type {
            if non_visited[y][x-1].2 {
                neighbors.push((x-1, y));
            }
        } else if !discount {
            perimeter += 1;
        }
        // right
        if x < self.width() - 1 && self.plants[y][x+1] == plant_type {
            if non_visited[y][x+1].2 {
                neighbors.push((x+1, y));
            }
        } else if !discount {
            perimeter += 1;
        }
        // up
        if y > 0 && self.plants[y-1][x] == plant_type {
            if non_visited[y-1][x].2 {
                neighbors.push((x, y-1));
            }
        } else if !discount {
            perimeter += 1;
        }
        // down
        if y < self.height() - 1 && self.plants[y+1][x] == plant_type {
            if non_visited[y+1][x].2 {
                neighbors.push((x, y+1));
            }
        } else if !discount {
            perimeter += 1;
        }

        if discount {
            if self.is_top_left_corner(x, y) {
                perimeter += 1;
            }
            if self.is_bottom_left_corner(x, y) {
                perimeter += 1;
            }
            if self.is_top_right_corner(x, y) {
                perimeter += 1;
            }
            if self.is_bottom_right_corner(x, y) {
                perimeter += 1;
            }
            if self.is_inward_top_left_corner(x, y) {
                perimeter += 1;
            }
            if self.is_inward_top_right_corner(x, y) {
                perimeter += 1;
            }
            if self.is_inward_bottom_left_corner(x, y) {
                perimeter += 1;
            }
            if self.is_inward_bottom_right_corner(x, y) {
                perimeter += 1;
            }
        }
        
        (perimeter, neighbors)
    }

    fn is_top_left_corner(&self, x: usize, y: usize) -> bool {
        let plant_type = self.plants[y][x];
        (x == 0 || self.plants[y][x-1] != plant_type) && (y == 0 || self.plants[y-1][x] != plant_type)
    }

    fn is_bottom_left_corner(&self, x: usize, y: usize) -> bool {
        let plant_type = self.plants[y][x];
        (x == 0 || self.plants[y][x-1] != plant_type) && (y == self.height()-1 || self.plants[y+1][x] != plant_type)
    }

    fn is_top_right_corner(&self, x: usize, y: usize) -> bool {
        let plant_type = self.plants[y][x];
        (x == self.width()-1 || self.plants[y][x+1] != plant_type) && (y == 0 || self.plants[y-1][x] != plant_type)
    }

    fn is_bottom_right_corner(&self, x: usize, y: usize) -> bool {
        let plant_type = self.plants[y][x];
        (x == self.width()-1 || self.plants[y][x+1] != plant_type) && (y == self.height()-1 || self.plants[y+1][x] != plant_type)
    }

    fn is_inward_top_left_corner(&self, x: usize, y: usize) -> bool {
        let plant_type = self.plants[y][x];
        x > 0 && self.plants[y][x-1] == plant_type && y > 0 && self.plants[y-1][x] == plant_type && self.plants[y-1][x-1] != plant_type
    }

    fn is_inward_top_right_corner(&self, x: usize, y: usize) -> bool {
        let plant_type = self.plants[y][x];
        x < self.width() - 1 && self.plants[y][x+1] == plant_type && y > 0 && self.plants[y-1][x] == plant_type && self.plants[y-1][x+1] != plant_type
    }

    fn is_inward_bottom_left_corner(&self, x: usize, y: usize) -> bool {
        let plant_type = self.plants[y][x];
        x > 0 && self.plants[y][x-1] == plant_type && y < self.height()-1 && self.plants[y+1][x] == plant_type && self.plants[y+1][x-1] != plant_type
    }

    fn is_inward_bottom_right_corner(&self, x: usize, y: usize) -> bool {
        let plant_type = self.plants[y][x];
        x < self.width() - 1 && self.plants[y][x+1] == plant_type && y < self.height()-1 && self.plants[y+1][x] == plant_type && self.plants[y+1][x+1] != plant_type
    }
}

impl From<&str> for Garden {
    fn from(input: &str) -> Self {
        let map = input.lines().map(|line| 
            line.chars().collect()
        ).collect();
        Self { plants: map }
    }
}

fn main() {
    let input = include_str!("../../input/day-12");
    let garden = Garden::from(input);
    let price = garden.compute_regular_fencing_price();
    println!("Price: {}", price);

    let price_with_discount = garden.compute_fencing_price_with_discount();
    println!("Price with discount: {}", price_with_discount);
}

#[cfg(test)]
mod tests {
    use crate::Garden;


    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day-12-test");
        let garden = Garden::from(input);
        let price = garden.compute_regular_fencing_price();
        assert_eq!(price, 1930);
    }
    
    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day-12-test");
        let garden = Garden::from(input);
        let price = garden.compute_fencing_price_with_discount();
        assert_eq!(price, 1206);
    }

    #[test]
    fn test_part2_2() {
        let input = include_str!("../../input/day-12-test-2");
        let garden = Garden::from(input);
        let price = garden.compute_fencing_price_with_discount();
        assert_eq!(price, 436);
    }

    #[test]
    fn test_part2_3() {
        let input = include_str!("../../input/day-12-test-3");
        let garden = Garden::from(input);
        let price = garden.compute_fencing_price_with_discount();
        assert_eq!(price, 236);
    }

    #[test]
    fn test_part2_4() {
        let input = include_str!("../../input/day-12-test-4");
        let garden = Garden::from(input);
        let price = garden.compute_fencing_price_with_discount();
        assert_eq!(price, 368);
    }
}