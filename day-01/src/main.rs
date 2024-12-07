use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/day-01");
    let (list1, list2) = create_lists(input);
    let distance = compute_distance(list1.clone(), list2.clone());
    println!("Distance: {distance}");

    let similarity = compute_similarity(list1, list2);
    println!("Similarity: {similarity}");
}

fn create_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut list1 = vec![];
    let mut list2 = vec![];
    input.lines().for_each(|line| {
        let split: Vec<&str> = line.split("   ").collect();
        list1.push(split[0].parse::<u32>().unwrap());
        list2.push(split[1].parse::<u32>().unwrap());
    });
    (list1, list2)
}

fn compute_distance(mut list1: Vec<u32>, mut list2: Vec<u32>) -> u32 {
    list1.sort();
    list2.sort();
    list1.iter().zip(list2.iter()).map(|(item1, item2)| if item1 > item2 {item1 - item2} else {item2 - item1}).sum()
}

fn compute_similarity(list1: Vec<u32>, list2: Vec<u32>) -> u32 {
    let mut list2_hashmap: HashMap<u32, u32> = HashMap::new();
    list2.iter().for_each(|item| {
        list2_hashmap.entry(*item).and_modify(|count| *count += 1).or_insert(1);
    });
    list1.iter().map(|item| item * list2_hashmap.get(item).unwrap_or(&0)).sum()
}

#[cfg(test)]
mod tests {
    use crate::{compute_distance, compute_similarity, create_lists};

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day-01-test");
        let (list1, list2) = create_lists(input);
        let distance = compute_distance(list1, list2);
        assert_eq!(distance, 11);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day-01-test");
        let (list1, list2) = create_lists(input);
        let distance = compute_similarity(list1, list2);
        assert_eq!(distance, 31);
    }

}