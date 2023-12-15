use std::vec;

use itertools::Itertools;

advent_of_code::solution!(12);

fn get_possible_arangements(springs: Vec<char>) -> Vec<Vec<bool>> {
    let possible: Vec<Vec<bool>> = vec![];
    let mut current_spring = vec![];

    for (i, spring) in springs.iter().enumerate() {
        if *spring == '?' {
            let mut broken_variant = current_spring.clone();
            broken_variant.push('#');
            broken_variant.append(&mut springs[i..springs.len()].to_owned());

            possible.append(&mut get_possible_arangements(broken_variant));
        }
        else {
            if spring == '.' {
                current_spring.push(true);
            }
            else {
                current_spring.push(false);
            }
        }
    }
    possible.push(current_spring);

    possible
}

fn fits_critera(springs: Vec<bool>, groups: Vec<u32>) -> bool {
    let mut current_group_len = 0;
    let mut current_group_index = 0;

    for spring in springs {
        if spring == false {
            if groups[current_group_index] != current_group_len {
                return false;
            }
            current_group_len = 0;
            current_group_index += 1;

            if current_group_index >= groups.len() {
                return false;
            }
        }
        else {
            current_group_len += 1;
        }
    }

    if groups[current_group_index] != current_group_len {
        return false;
    }
    else {
        return true;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);
    dbg!(get_possible_arangements("???.###".chars().collect()));
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    None
}

fn parse(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
