use std::usize;

advent_of_code::solution!(3);

#[derive(Clone, Copy, Debug)]
struct EnginePart {
    number: u32,
    r: usize,
    c: usize,
    len: usize,
}

impl EnginePart {
    fn is_adjacent(self, r: usize, c: usize) -> bool {
        (self.r-1..self.r+2).contains(&r)
            && (self.c - 1..self.c + self.len + 1).contains(&c)
    }
}

fn is_part_number(schematic: &Vec<Vec<char>>,  r: usize, c: usize) -> bool {
    for r in r-1..r+2 {
        for c in c-1..c+2 {
            let char = schematic[r][c];
            if !char.is_digit(10) && char != '.' {
                return true;
            }
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);
    let mut total = 0;

    for (row_index, row) in input.iter().enumerate() {
        let mut buffer = String::from("");
        let mut is_part = false;
        
        for (col_index, col) in row.iter().enumerate() {
            if !col.is_digit(10) {                
                if is_part {
                    total += buffer.parse().unwrap_or(0);
                }

                buffer = String::from("");
                is_part = false;
            }
            else {
                buffer.push(*col);
                if is_part_number(&input, row_index, col_index) {
                    is_part = true;
                }
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    let mut parts: Vec<EnginePart> = vec![];
    let mut total = 0;

    for (row_index, row) in input.iter().enumerate() {
        let mut buffer = String::from("");
        
        for (col_index, col) in row.iter().enumerate() {
            if !col.is_digit(10) {
                let part_number: Result<u32, _> = buffer.parse();

                match part_number {
                    Ok(_) => {
                        parts.push(EnginePart {
                        number: buffer.parse().unwrap(),
                        r: row_index,
                        c: col_index - buffer.len(),
                        len: buffer.len(),
                        })
                    }
                    Err(_) => continue,
                };

                buffer = String::from("");
            }
            else {
                buffer.push(*col);
            }
        }
    }

    for (row_index, row) in input.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            if *col == '*' {
                let mut adjacent_parts: Vec<(EnginePart, bool)> = parts.clone().into_iter()
                    .map(|p| (p, p.is_adjacent(row_index, col_index))).collect();
                
                adjacent_parts.retain(|p| p.1 );

                if adjacent_parts.len() == 2 {
                    total += adjacent_parts[0].0.number * adjacent_parts[1].0.number;
                }
            }
        }
    }

    Some(total)
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let input: Vec<&str> = input.split("\n").collect();
    let mut parsed: Vec<Vec<char>> = vec![];
    let line_len = input[0].len();

    parsed.push(".".repeat(line_len).chars().collect());
 
    for line in input {
        let line = String::from(".") + line + ".";
        parsed.push(line.chars().collect());
    }

    parsed.push(".".repeat(line_len).chars().collect());

    parsed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
