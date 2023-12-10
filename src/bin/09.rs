use std::vec;

advent_of_code::solution!(9);

fn calc_diffs(line: &Vec<i32>) -> Vec<i32> {
    let mut diffs = vec![];

        for num in line.windows(2) {
            diffs.push(num[1] - num[0]);
        }

    diffs
}

fn predict_line_start(line: &Vec<i32>) -> i32 {
    let mut sequences: Vec<Vec<i32>> = vec![];
    let mut diffs = line.to_owned();
    
    while diffs.iter().any(|num| *num != 0) {
        sequences.push(diffs.clone());
        diffs = calc_diffs(&diffs);
    }
    
    diffs.push(0);
    sequences.push(diffs.clone());

    for i in (0..sequences.len() - 1).rev() {
        let increase = sequences[i][0] - sequences[i + 1][0];
        sequences[i].insert(0, increase);
    }
    
    sequences[0][0]
}

fn predict_line_end(line: &Vec<i32>) -> i32 {
    let mut sequences: Vec<Vec<i32>> = vec![];
    let mut diffs = line.to_owned();
    
    while diffs.iter().any(|num| *num != 0) {
        sequences.push(diffs.clone());
        diffs = calc_diffs(&diffs);
    }
    
    diffs.push(0);
    sequences.push(diffs.clone());

    for i in (0..sequences.len() - 1).rev() {
        let increase = sequences[i].last().unwrap() + sequences[i + 1].last().unwrap();
        sequences[i].push(increase);
    }
    
    *sequences[0].last().unwrap()
}

pub fn part_one(input: &str) -> Option<i32> {
    let input = parse(input);
    let total: i32 = input.iter().map(predict_line_end).sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<i32> {
    let input = parse(input);
    let total: i32 = input.iter().map(predict_line_start).sum();

    Some(total)
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input.split("\n").map(|line| line.split(" ").map(|number| number.parse().unwrap()).collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
