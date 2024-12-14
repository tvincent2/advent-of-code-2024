struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn compute_position_after_n_seconds(&self, seconds: i32, width: i32, height: i32) -> (i32, i32) {
        let new_x = (self.x + self.vx * seconds).rem_euclid(width);
        let new_y = (self.y + self.vy * seconds).rem_euclid(height);
        (new_x, new_y)
    }
}

impl From<&str> for Robot {
    fn from(input: &str) -> Self {
        let split: Vec<&str> = input.split(' ').collect();
        let position: Vec<&str> = split[0].split('=').collect();
        let coordinates: Vec<i32> = position[1].split(',').map(|c| c.parse::<i32>().unwrap()).collect();
        let x = coordinates[0];
        let y = coordinates[1];

        let velocity: Vec<&str> = split[1].split('=').collect();
        let vcoordinates: Vec<i32> = velocity[1].split(',').map(|c| c.parse::<i32>().unwrap()).collect();
        let vx = vcoordinates[0];
        let vy = vcoordinates[1];

        Self { x, y, vx, vy }
    }
}

struct Robots {
    robots: Vec<Robot>,
    width: i32,
    height: i32,
}

impl Robots {
    fn compute_safety_factor(&self) -> u32 {
        let h_middle = self.height / 2;
        let v_middle = self.width / 2;
        
        let robots_after_100_seconds: Vec<(i32, i32)> = self.robots.iter().map(|robot|
            robot.compute_position_after_n_seconds(100, self.width, self.height)
        ).collect();

        let mut in_ne_quadrant = 0;
        let mut in_nw_quadrant = 0;
        let mut in_se_quadrant = 0;
        let mut in_sw_quadrant = 0;
        robots_after_100_seconds.into_iter().for_each(|(x, y)| {
            if x < v_middle && y < h_middle {
                in_nw_quadrant += 1;
            } else if x > v_middle && y < h_middle {
                in_ne_quadrant += 1;
            } else if x < v_middle && y > h_middle {
                in_sw_quadrant += 1;
            } else if x > v_middle && y > h_middle {
                in_se_quadrant += 1;
            }
        });

        in_ne_quadrant * in_nw_quadrant * in_se_quadrant * in_sw_quadrant
    }

    fn get_positions_after_n_seconds(&self, n: i32) -> Vec<(i32, i32)> {
        self.robots.iter().map(|robot| {
            robot.compute_position_after_n_seconds(n, self.width, self.height)
        }).collect()
    }
    
    fn display(&self, positions: Vec<(i32, i32)>) {
        for line_index in 0..self.height {
            let mut line = "".to_string();
            for col_index in 0..self.width {
                if positions.contains(&(col_index, line_index)) {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            println!("{line}");
        }
        println!();
    }

    fn save_image(&self, positions: Vec<(i32, i32)>, index: i32) {
        let mut imgbuf = image::ImageBuffer::new(self.width as u32, self.height as u32);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            if positions.contains(&(x as i32, y as i32)) {
                *pixel = image::Rgb([0, 255u8, 0]);
            }
        }
        let path = format!("day-14/images/{}.png", index);
        imgbuf.save(path).unwrap();
    }
}

impl From<&str> for Robots {
    fn from(input: &str) -> Self {
        let robots: Vec<Robot> = input.lines().map(Robot::from).collect();
        let width = robots.iter().map(|robot| robot.x).max().unwrap() + 1;
        let height = robots.iter().map(|robot| robot.y).max().unwrap() + 1;
        Self { robots, width, height }
    }
}

fn main() {
    let input = include_str!("../../input/day-14");
    let robots = Robots::from(input);
    let safety_factor = robots.compute_safety_factor();
    println!("Safety factor: {}", safety_factor);

    // good luck browsing everything
    for n in 0..10403 {
        let positions = robots.get_positions_after_n_seconds(n);
        robots.save_image(positions, n);
    }
    println!("Done");
}

#[cfg(test)]
mod tests {
    use crate::{Robot, Robots};

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day-14-test");
        let robots = Robots::from(input);
        let safety_factor = robots.compute_safety_factor();
        assert_eq!(safety_factor, 12);
    }

    #[test]
    fn test_part1_robot_move() {
        let robot = Robot { x: 2, y: 4, vx: 2, vy: -3 };
        let position = robot.compute_position_after_n_seconds(1, 11, 7);
        assert_eq!(position, (4, 1));
        let position = robot.compute_position_after_n_seconds(2, 11, 7);
        assert_eq!(position, (6, 5));
        let position = robot.compute_position_after_n_seconds(3, 11, 7);
        assert_eq!(position, (8, 2));
        let position = robot.compute_position_after_n_seconds(4, 11, 7);
        assert_eq!(position, (10, 6));
        let position = robot.compute_position_after_n_seconds(5, 11, 7);
        assert_eq!(position, (1, 3));
    }
}