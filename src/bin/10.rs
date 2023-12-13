use std::{collections::HashMap, ops::Index};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);
    let num_of_pipes = input.iter().flatten().filter(|pipe| **pipe != '.').count() as u32;
    Some(num_of_pipes / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    None
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let starting_index = input.chars().position(|char| char == 'S').expect("S should be in input as it denotes the starting position");
    let mut input: Vec<Vec<char>> = input.split("\n").map(|line| String::from(".") + line + ".").map(|line| line.chars().collect()).collect();
    input.insert(0, ".".repeat(input[0].len()).chars().collect());
    input.push(".".repeat(input[0].len()).chars().collect());
    let mut pipes = vec![vec!['.'; input[0].len()]; input.len()];

    // [north, east, south, west]
    let mut pipe_lookup = HashMap::new();
    pipe_lookup.insert('|', [true, false, true, false]);
    pipe_lookup.insert('-', [false, true, false, true]);
    pipe_lookup.insert('L', [true, true, false, false]);
    pipe_lookup.insert('J', [true, false, false, true]);
    pipe_lookup.insert('7', [false, false, true, true]);
    pipe_lookup.insert('F', [false, true, true, false]);

    let start_row = input.iter().map(|row| row.contains(&'S')).position(|row_contains| row_contains == true).expect("S should be somewhere");
    let start_col = input[start_row].iter().position(|pipe| *pipe == 'S').expect("S should be somewhere");
    //let (start_row, start_col) = (starting_index / (input.len() - 2) + 1, starting_index % (input[0].len() - 2) + 1);
    let (mut row, mut col) = (start_row, start_col);
    let mut last_dir = None;
    pipes[row][col] = 'S';
    dbg!(input[row][col]);
    assert!(input[row][col] == 'S');
    let south = pipe_lookup.get(&input[row + 1][col]).unwrap_or(&[false, false, false, false]);
    let east = pipe_lookup.get(&input[row][col + 1]).unwrap_or(&[false, false, false, false]);
    let west = pipe_lookup.get(&input[row][col.saturating_sub(1)]).unwrap_or(&[false, false, false, false]);
    let north = pipe_lookup.get(&input[row.saturating_sub(1)][col]).unwrap_or(&[false, false, false, false]);

    if north[2] == true {
        row -= 1;
        last_dir = Some(2);
    }
    else if east[3] == true {
        col += 1;
        last_dir = Some(3);
    }
    else if south[0] == true {
        row += 1;
        last_dir = Some(0);
    }
    else if west[1] == true {
        col -= 1;
        last_dir = Some(1);
    }
    else {
        panic!("No way to go");
    }
    
    loop {
        let pipe = input[row][col];
        pipes[row][col] = pipe;
        //dbg!(pipe);
        let mut directions = pipe_lookup.get(&pipe).unwrap().clone();
        directions[last_dir.unwrap()] = false;
        
        last_dir = directions.iter().position(|v| *v == true);


        if last_dir == Some(0) {
            row -= 1;
            last_dir = Some(2);
        }
        else if last_dir == Some(1) {
            col += 1;
            last_dir = Some(3);
        }
        else if last_dir == Some(2) {
            row += 1;
            last_dir = Some(0);
        }
        else if last_dir == Some(3) {
            col -= 1;
            last_dir = Some(1);
        }
        else {
            panic!("No way to go");
        }

        if row == start_row && col == start_col {
            break;
        }
    }
    

    pipes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(4));

        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
