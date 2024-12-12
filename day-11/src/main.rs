use std::collections::HashMap;

struct Arrangement {
    stones: Vec<u64>
}

impl Arrangement {
    fn blink(&mut self) {
        self.stones = self.stones.iter().flat_map(|stone| transform(*stone)).collect();
    }

    fn blink_25_times_and_count(&mut self) -> usize {
        (0..25).for_each(|_| self.blink());
        self.stones.len()
    }

    fn blink_50_times_and_count(&mut self) -> usize {
        let mut cache_5 = HashMap::new();
        let mut cache_25 = HashMap::new();
        let mut cache_50 = HashMap::new();
        let mut counter = 0;
        self.stones.iter()
            .map(|stone| {
                counter += 1;
                let count = if let Some(count) = cache_50.get(stone) {
                    *count
                } else {
                    let count = transform_50_times_and_count(*stone, &mut cache_5, &mut cache_25);
                    cache_50.insert(stone, count);
                    count
                };
                count
            }).sum()
    }

    fn print_count_unique(&self) {
        let mut stones = self.stones.clone();
        stones.sort();
        stones.dedup();
        println!("size: {}", stones.len());
    }
}

fn transform(stone: u64) -> Vec<u64> {
    if stone == 0 {
        vec![1]
    } else {
        let mut stone_string = stone.to_string();
        if stone_string.len() % 2 == 0 {
            let split = stone_string.split_off(stone_string.len() / 2);
            vec![stone_string.parse::<u64>().unwrap(), split.parse::<u64>().unwrap()]
        } else {
            vec![stone * 2024]
        }
    }
}

fn transform_5_times(stone: u64) -> Vec<u64> {
    let mut result = vec![stone];
    (0..5).for_each(|_| result = result.iter().flat_map(|stone| transform(*stone)).collect());
    result
}

fn transform_50_times_and_count(stone: u64, cache_5: &mut HashMap<u64, Vec<u64>>, cache_25: &mut HashMap<u64, Vec<u64>>) -> usize {
    let stones_after_25 = if let Some(stones) = cache_25.get(&stone) {
        stones.clone()
    } else {
        let mut stones = vec![stone];
        (0..5).for_each(|_| {
            stones = stones.iter().flat_map(|stone| {
                if let Some(stones) = cache_5.get(stone) {
                    stones.clone()
                } else {
                    let stones = transform_5_times(*stone);
                    cache_5.insert(*stone, stones.clone());
                    stones
                }
            }).collect();
        });
        cache_25.insert(stone, stones.clone());
        stones
    };
    let count_after_50 = stones_after_25.iter()
        .map(|&stone| {
            if let Some(stones) = cache_25.get(&stone) {
                stones.len()
            } else {
                let mut stones = vec![stone];
                (0..5).for_each(|_| {
                    stones = stones.iter().flat_map(|stone| {
                        if let Some(stones) = cache_5.get(stone) {
                            stones.clone()
                        } else {
                            let stones = transform_5_times(*stone);
                            cache_5.insert(*stone, stones.clone());
                            stones
                        }
                    }).collect();
                });
                cache_25.insert(stone, stones.clone());
                stones.len()
            }
        }).sum();
    count_after_50
}

impl From<&str> for Arrangement {
    fn from(input: &str) -> Self {
        let stones = input.split(' ').map(|value| value.parse::<u64>().unwrap()).collect();
        Self { stones }
    }
}

fn main() {
    let input = include_str!("../../input/day-11");
    let mut arrangement = Arrangement::from(input);
    let count_25 = arrangement.blink_25_times_and_count();
    println!("Stones after 25 blinks: {count_25}");
    arrangement.print_count_unique();

    // should re-init ?

    let count_75 = arrangement.blink_50_times_and_count();
    println!("Stones after 75 blinks: {count_75}");
}

#[cfg(test)]
mod tests {
    use crate::Arrangement;


    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day-11-test");
        let mut arrangement = Arrangement::from(input);
        let count = arrangement.blink_25_times_and_count();
        assert_eq!(count, 55312);
    }
}