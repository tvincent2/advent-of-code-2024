use std::collections::HashMap;

fn numeric_keypad_button_to_position(button: char) -> (i32, i32) {
    match button {
        'A' => (2, 3),
        '0' => (1, 3),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        _ => unreachable!()
    }
}

fn arrow_keypad_button_to_position(button: char) -> (i32, i32) {
    match button {
        'A' => (2, 0),
        '^' => (1, 0),
        '<' => (0, 1),
        'v' => (1, 1),
        '>' => (2, 1),
        _ => unimplemented!(),
    }
}

fn paths_to_next_coordinate_numeric((ax, ay): (i32, i32), (bx, by): (i32, i32)) -> Vec<String> {
    let dx = bx - ax;
    let dy = by - ay;
    if dx != 0 || dy != 0 {
        let mut result = vec![];
        let mut horizontal_path = "".to_string();
        if dx > 0 {
            for _ in 0..dx { horizontal_path.push('>'); }
        }
        if dx < 0 {
            for _ in 0..dx.abs() { horizontal_path.push('<'); }
        }
    
        let mut vertical_path = "".to_string();
        if dy > 0 {
            for _ in 0..dy { vertical_path.push('v'); }
        }
        if dy < 0 {
            for _ in 0..dy.abs() { vertical_path.push('^'); }
        }
        if ay != 3 || bx != 0 { 
            result.push(format!("{}{}A", &horizontal_path, &vertical_path));
        }
        if by != 3 || ax != 0 {
            result.push(format!("{}{}A", &vertical_path, &horizontal_path));
        }
        result
    } else {
        vec!["A".to_string()]
    }
}

fn paths_to_next_coordinate_arrow((ax, ay): (i32, i32), (bx, by): (i32, i32)) -> Vec<String> {
    let dx = bx - ax;
    let dy = by - ay;
    if dx != 0 || dy != 0 {
        let mut result = vec![];
        let mut horizontal_path = "".to_string();
        if dx > 0 {
            for _ in 0..dx { horizontal_path.push('>'); }
        }
        if dx < 0 {
            for _ in 0..dx.abs() { horizontal_path.push('<'); }
        }
    
        let mut vertical_path = "".to_string();
        if dy > 0 {
            for _ in 0..dy { vertical_path.push('v'); }
        }
        if dy < 0 {
            for _ in 0..dy.abs() { vertical_path.push('^'); }
        }
        if ay != 0 || bx != 0 { 
            result.push(format!("{}{}A", &horizontal_path, &vertical_path));
        }
        if by != 0 || ax != 0 {
            result.push(format!("{}{}A", &vertical_path, &horizontal_path));
        }
        result.sort();
        result.dedup();
        result
    } else {
        vec!["A".to_string()]
    }
}

fn arrow_to_shortest_path(button1: char, button2: char, level: usize, cache: &mut HashMap<(char, char, usize), usize>) -> usize {
    let paths = paths_to_next_coordinate_arrow(arrow_keypad_button_to_position(button1), arrow_keypad_button_to_position(button2));
    if level == 0 {
        paths.iter().map(|path| path.len()).min().unwrap()
    } else {
        let path = paths.iter().map(|path| {
            let buttons: Vec<char> = ['A'].into_iter().chain(path.chars()).collect();
            buttons.windows(2).map(|buttons| {
                let button1 = buttons[0];
                let button2 = buttons[1];
                if let Some(&len) = cache.get(&(button1, button2, level - 1)) {
                    len
                } else {
                    let len = arrow_to_shortest_path(button1, button2, level - 1, cache);
                    cache.insert((button1, button2, level - 1), len);
                    len
                }
            }).sum()
        }).min().unwrap();
        path
    }
}

fn numeric_to_shortest_final_path(button1: char, button2: char, level: usize) -> usize {
    let paths = paths_to_next_coordinate_numeric(numeric_keypad_button_to_position(button1), numeric_keypad_button_to_position(button2));
    //println!("{} -> {} has potential paths {:?}", button1, button2, paths);
    let mut cache: HashMap<(char, char, usize), usize> = HashMap::new();
    paths.iter().map(|path| {
        let buttons: Vec<char> = ['A'].into_iter().chain(path.chars()).collect();
        buttons.windows(2).map(|buttons|{
            let button1 = buttons[0];
            let button2 = buttons[1];
            if let Some(&len) = cache.get(&(button1, button2, level-1)) {
                len
            } else {
                let len = arrow_to_shortest_path(button1, button2, level - 1, &mut cache);
                cache.insert((button1, button2, level-1), len);
                len
            }
        }).sum()
    }).min().unwrap()
}

fn numeric_sequence_to_final_sequence_length_part1(sequence: String) -> usize {
    let buttons: Vec<char> = ['A'].into_iter().chain(sequence.chars()).collect();
    buttons.windows(2).map(|buttons|{
        let button1 = buttons[0];
        let button2 = buttons[1];
        numeric_to_shortest_final_path(button1, button2, 2)
    }).sum()
}

fn numeric_sequence_to_final_sequence_length_part2(sequence: String) -> usize {
    let buttons: Vec<char> = ['A'].into_iter().chain(sequence.chars()).collect();
    buttons.windows(2).map(|buttons|{
        let button1 = buttons[0];
        let button2 = buttons[1];
        numeric_to_shortest_final_path(button1, button2, 25)
    }).sum()
}

fn sequence_numeric_part(sequence: &str) -> usize {
    sequence[0..3].parse::<usize>().expect("invalid numeric sequence")
}

fn sequence_to_complexity_part1(sequence: String) -> usize {
    let numeric_value = sequence_numeric_part(&sequence);
    let len = numeric_sequence_to_final_sequence_length_part1(sequence);
    len * numeric_value
}

fn sequence_to_complexity_part2(sequence: String) -> usize {
    let numeric_value = sequence_numeric_part(&sequence);
    let len = numeric_sequence_to_final_sequence_length_part2(sequence);
    len * numeric_value
}

fn read_sequences(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

fn main() {
    let input = include_str!("../../input/day-21");
    let sequences = read_sequences(input);
    let sum: usize = sequences.clone().into_iter().map(|sequence| sequence_to_complexity_part1(sequence)).sum();
    println!("Complexity sum: {}", sum);

    let sum2: usize = sequences.into_iter().map(|sequence| sequence_to_complexity_part2(sequence)).sum();
    println!("Complexity sum part 2: {}", sum2);
}

#[cfg(test)]
mod tests {
    use crate::{numeric_sequence_to_final_sequence_length_part1, read_sequences, sequence_to_complexity_part1};


    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day-21-test");
        let sequences = read_sequences(input);
        let sum: usize = sequences.into_iter().map(|sequence| sequence_to_complexity_part1(sequence)).sum();
        assert_eq!(sum, 126384);
    }

    #[test]
    fn test_part1_final_sequence() {
        let input = include_str!("../../input/day-21-test");
        let sequences = read_sequences(input);
        let final_sequence0 = numeric_sequence_to_final_sequence_length_part1(sequences[0].clone());
        println!("{}", final_sequence0);
        assert_eq!(final_sequence0, 68);
    }
    
    #[test]
    fn test_part2() {
    }
}