use std::vec;

use itertools::Itertools;

advent_of_code::solution!(12);

fn get_possible_arangements(springs: Vec<char>) -> Vec<Vec<bool>> {
    let mut possible: Vec<Vec<bool>> = vec![];
    let mut current_spring = vec![];
    let mut  current_spring_chars = vec![];
    for (i, spring) in springs.iter().enumerate() {
        if *spring == '?' {
            //current_spring_chars.push('.');
            current_spring.push(false);

            let mut broken_variant = current_spring_chars.clone();
            broken_variant.push('#');
            broken_variant.append(&mut springs[i + 1..springs.len()].to_owned());

            possible.append(&mut get_possible_arangements(broken_variant));
            current_spring_chars.push('.');

        }
        else {
            current_spring_chars.push(*spring);

            if *spring == '.' {
                current_spring.push(false);
            }
            else {
                current_spring.push(true);
            }
        }
    }
    possible.push(current_spring);

    possible
}

fn fits_critera(springs: &Vec<bool>, groups: &Vec<u32>) -> bool {
    let mut current_group_len = 0;
    let mut current_group_index = 0;
    let mut had_last_group = false;

    for spring in springs {
        if *spring == false {
            if current_group_len > 0 {
                if current_group_index > groups.len() || groups.get(current_group_index) != Some(&current_group_len) {
                    return false;
                }
                
                if current_group_index == groups.len() - 1 {
                    had_last_group = true;
                }

                current_group_len = 0;
                current_group_index += 1;
            }
        }
        else {
            if had_last_group {
                return false;
            }

            current_group_len += 1;
        }
    }

    if current_group_index == groups.len() {
        return true;
    }
    else if current_group_index == groups.len() - 1 && groups[current_group_index] == current_group_len {
        return true;
    }
    else {
        return false;
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    let mut total = 0;

    for (springs, groups) in input.iter() {
        total += get_possible_arangements(springs.chars().collect()).iter()
            .map(|springs| fits_critera(springs, groups))
            .filter(|result| *result).count();
    }

    ////dbg!(get_possible_arangements("..#...#...###.".chars().collect()));

    // //dbg!(fits_critera(&vec![ false,
    //     true,
    //     true,
    //     true,
    //     false,
    //     false,
    //     true,
    //     true,
    //     false,
    //     true,
    //     false,
    //     true,], &vec![3, 2, 1]));
    ////dbg!(get_possible_arangements("?.?".chars().collect()));
    // let arrangements = get_possible_arangements("?###????????".chars().collect());
    // let results = arrangements.clone().iter()
    //     .map(|springs| fits_critera(springs, &vec![3, 2, 1]))
    //     .collect::<Vec<_>>();

    // for i in 0..arrangements.len() {
    //     if results[i] == true {
    //         //dbg!(&arrangements[i]);
    //     }
    // }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    None
}

fn parse(input: &str) -> Vec<(&str, Vec<u32>)> {
    let input: Vec<_> = input.split("\n").collect();
    let mut springs: Vec<(&str, Vec<u32>)> = vec![];

    for set in input {
        let mut set = set.split(" ");
        springs.push((set.next().unwrap(), set.next().unwrap().split(",").map(|group| group.parse().unwrap()).collect()));
    }

    springs
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
