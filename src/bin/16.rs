use std::collections::HashSet;
use std::cmp::max;

advent_of_code::solution!(16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn apply_movement(r: isize, c: isize, direction: Direction) -> (isize, isize) {
    let r = r as isize + match direction {
        Direction::Up => -1,
        Direction::Down => 1,
        _ => 0,
    };

    let c = c as isize + match direction {
        Direction::Left => -1,
        Direction::Right => 1,
        _ => 0,
    };

    (r, c)
}

fn update_energy_map(contraption: &Vec<Vec<char>>, energy_map: &mut Vec<Vec<bool>>,
    visited: &mut HashSet<(isize, isize, Direction)>, mut r: isize, mut c: isize, mut direction: Direction ) {

    let max_r = contraption.len() as isize;
    let max_c = contraption[0].len() as isize;

    loop {
        let state = (r, c, direction);

        if visited.contains(&state) {
            break;
        }
        visited.insert(state);

        (r, c) = apply_movement(r, c, direction);
        
        if r < 0 || r >= max_r || c < 0 || c >= max_c {
            break;
        }

        energy_map[r as usize][c as usize] = true;

        let item = contraption[r as usize][c as usize];

        if item == '/' {
            direction = match direction {
                Direction::Down => Direction::Left,
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Up,
                Direction::Left => Direction::Down,
            }
        }
        else if item == '\\' {
            direction = match direction {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            }
        }
        else if item == '-' {
            direction = match direction {
                Direction::Left => Direction::Left,
                Direction::Right => Direction::Right,
                Direction::Up => {
                    update_energy_map(contraption, energy_map, visited, r, c, Direction::Right);
                    Direction::Left
                },
                Direction::Down => {
                    update_energy_map(contraption, energy_map, visited, r, c, Direction::Right);
                    Direction::Left
                },
            }
        }
        else if item == '|' {
            direction = match direction {
                Direction::Up => Direction::Up,
                Direction::Down => Direction::Down,
                Direction::Left => {
                    update_energy_map(contraption, energy_map, visited, r, c, Direction::Up);
                    Direction::Down
                },
                Direction::Right => { 
                    update_energy_map(contraption, energy_map, visited ,r, c, Direction::Up);
                    Direction::Down
                },
            }
        }
    }
}

fn energised_tiles(input: &Vec<Vec<char>>, r: isize, c: isize, direction: Direction) -> usize {
    let mut energy_map: Vec<Vec<bool>> = vec![vec![false; input[0].len()]; input.len()];
    let mut visted_cache: HashSet<(isize, isize, Direction)> = HashSet::new();

    update_energy_map(&input, &mut energy_map, &mut visted_cache, r, c, direction);
    
    let total = energy_map.iter().map(|row| row.iter().filter(|&n| *n == true).count()).sum();
    total
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    Some(energised_tiles(&input, 0, -1, Direction::Right))
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);
    let mut highest = 0;

    for r in 0..input.len() {
        let right = energised_tiles(&input, r as isize, -1, Direction::Right);
        let left = energised_tiles(&input, r as isize, input[0].len() as isize + 1, Direction::Left);
        highest = max(highest, max(left, right));
    }

    for c in 0..input[0].len() {
        let down = energised_tiles(&input, -1, c as isize, Direction::Down);
        let up = energised_tiles(&input, input.len() as isize + 1, c as isize, Direction::Up);
        highest = max(highest, max(up, down));
    }

    Some(highest)
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.split("\n").map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
