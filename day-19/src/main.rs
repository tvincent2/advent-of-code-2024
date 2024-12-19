use std::collections::HashMap;

struct Onsen {
    towels: Vec<String>,
    patterns: Vec<String>,
}

impl Onsen {
    fn count_possible_patterns(&self) -> usize {
        let possible: Vec<_> = self.patterns.iter().filter(|pattern| {
            is_pattern_possible_rec(pattern, &self.towels, 0)
        }).collect();
        possible.len()
    }

    fn count_all_possible_combinations(&self) -> usize {
        self.patterns.iter().map(|pattern| {
            let mut cache: HashMap<usize, usize> = HashMap::new();
            all_possible_patterns(pattern, &self.towels, 0, &mut cache)
        }
        ).sum()
    }
}

fn is_pattern_possible_rec(pattern: &str, towels: &Vec<String>, index: usize) -> bool {
    index == pattern.len() ||
        towels.iter().any(|towel| pattern[index..].starts_with(towel) && is_pattern_possible_rec(pattern, towels, index + towel.len()))
}

fn all_possible_patterns(pattern: &str, towels: &Vec<String>, index: usize, cache: &mut HashMap<usize, usize>) -> usize {
    if index == pattern.len() {
        1
    } else {
        if let Some(count) = cache.get(&index) {
            return *count;
        } else {
            let count = towels.iter().map(|towel| {
                if pattern[index..].starts_with(towel) {
                    all_possible_patterns(pattern, towels, index + towel.len(), cache)
                } else {
                    0
                }
            }).sum();
            cache.insert(index, count);
            return count;
        }
    }
}

impl From<&str> for Onsen {
    fn from(input: &str) -> Self {
        let towels = input.lines().take(1).map(|line| line.split(", ").map(|t| t.to_string()).collect()).next().unwrap();
        let patterns = input.lines().skip(2).map(|line| line.to_string()).collect();
        Self { towels, patterns }
    }
}

fn main() {
    let input = include_str!("../../input/day-19");
    let onsen = Onsen::from(input);
    let possible_patterns = onsen.count_possible_patterns();
    println!("Possible patterns: {}", possible_patterns);

    let combinations = onsen.count_all_possible_combinations();
    println!("All possible combinations: {}", combinations);
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, vec};
    use crate::{all_possible_patterns, is_pattern_possible_rec, Onsen};

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day-19-test");
        let onsen = Onsen::from(input);
        let possible_patterns = onsen.count_possible_patterns();
        assert_eq!(possible_patterns, 6);
    }

    #[test]
    fn test_part1_pattern() {
        let pattern = "brwrr";
        let towels = vec!["r".to_string(), "wr".to_string(), "b".to_string(), "g".to_string(), "bwu".to_string(), "rb".to_string(), "gb".to_string(), "br".to_string()];
        assert!(is_pattern_possible_rec(pattern, &towels, 0));
    }
    
    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day-19-test");
        let onsen = Onsen::from(input);
        let combinations = onsen.count_all_possible_combinations();
        assert_eq!(combinations, 16);
    }

    #[test]
    fn test_part2_pattern() {
        let pattern = "bggr";
        let towels = vec!["r".to_string(), "wr".to_string(), "b".to_string(), "g".to_string(), "bwu".to_string(), "rb".to_string(), "gb".to_string(), "br".to_string()];
        let mut cache = HashMap::new();
        assert_eq!(all_possible_patterns(pattern, &towels, 0, &mut cache), 1);
    }
}