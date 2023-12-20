use itertools::repeat_n;
use std::cmp::min;

advent_of_code::solution!(13);

fn has_reflection_on_line(pattern: &Vec<u32>, line: usize) -> usize {
    let mut reflects = true;
    //dbg!(pattern);
    if (pattern.len() + 1) % 2 == 0 {
        let mut i = 0;
        loop {
            if i > line {
                break;
            }

            if pattern.get(line - i) == None || pattern.get(line + i + 1) == None {
                break;
            }

            if pattern.get(line - i) != pattern.get(line + i + 1) {
                reflects = false;
            }
            i += 1;
        }

        if reflects {
            return line + 1;
        }
    }
    
    return 0;
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    let mut total = 0;
    //dbg!(&input[2]);
    dbg!(has_reflection_on_line(&input[2].0, 1));
    for (i, pattern) in input.iter().enumerate() {
        let rows = &pattern.0;
        let cols = &pattern.1;

        for i in 1..rows.len() - 2 {
            let reflect_rows =  has_reflection_on_line(&rows, i);
            //dbg!(i);
            total += reflect_rows * 100;

            if reflect_rows > 0 {
                dbg!(i);
                break;
            }
        
        }

        for i in 1..cols.len() - 2 {
            let reflect_cols = has_reflection_on_line(&cols, i);
            //dbg!(i);
            total += reflect_cols;

            if reflect_cols > 0 {
                dbg!(i);
                break;
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    None
}

fn parse(input: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    let input = input.split("\n\n").map(|line| line.split("\n"));   
    let mut patterns = vec![];

    for pattern in input {
        let mut rows = vec![];
        let mut cols: Vec<u32> = repeat_n(0, pattern.clone().next().unwrap().chars().count()).collect();

        for (row_i, line) in pattern.enumerate() {
            let mut row: u32 = 0;

            for (col_i, item) in line.chars().enumerate() {
                if item == '#' {
                    cols[col_i] |= 1 << row_i;
                    row |= 1 << col_i;
                }
            }

            rows.push(row);
        }

        patterns.push((rows, cols));
    }

    patterns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
