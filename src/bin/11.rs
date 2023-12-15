use itertools::Itertools;

advent_of_code::solution!(11);

fn get_manhattan_distance(coords1: (usize, usize), coords2: (usize, usize)) -> usize {
    ((coords1.0 as isize - coords2.0 as isize).abs() + (coords1.1 as isize - coords2.1 as isize).abs()) as usize
}

fn get_galaxy_indicies(map: &Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    let mut indicies = vec![];

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == true {
                indicies.push((row, col));
            }
        }
    }

    indicies
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input, 1);
    let pairs = input.iter().combinations(2);
    let distances = pairs.map(|coords| get_manhattan_distance(*coords[0], *coords[1]));

    Some(distances.sum::<usize>())
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input, 1_000_000 - 1);
    let pairs = input.iter().combinations(2);
    let distances = pairs.map(|coords| get_manhattan_distance(*coords[0], *coords[1]));

    Some(distances.sum::<usize>())
}

fn parse(input: &str, change_for_empty: usize) -> Vec<(usize, usize)> {
    let input: Vec<Vec<bool>> = input.split("\n").map(|row| row.chars().map(|space| space == '#').collect()).collect();
    let mut rows_needed = vec![];
    let mut cols_needed = vec![];

    for (i, row) in input.iter().enumerate() {
        if row.iter().all(|space| !space) {
            rows_needed.push(i);
        }
    }

    for col_index in 0..input[0].len() {
        let mut col = input.iter()
            .map(|s| s.iter().nth(col_index).unwrap());

        if col.all(|space| !*space) {
            cols_needed.push(col_index);
        }
    }
    
    let mut input = get_galaxy_indicies(&input);

    for galaxy in input.iter_mut() {
        galaxy.0 += rows_needed.iter().filter(|row| **row < galaxy.0).count() * change_for_empty;
        galaxy.1 += cols_needed.iter().filter(|col| **col < galaxy.1).count() * change_for_empty;
    }

    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
