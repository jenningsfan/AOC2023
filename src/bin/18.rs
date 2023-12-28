use std::vec;

advent_of_code::solution!(18);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl Direction {
    fn apply_position(&self, start_pos: Position) -> Position {
        match self {
            Self::Up(steps) => {
                Position {
                    row: start_pos.row - steps,
                    col: start_pos.col,
                }
            },
            Self::Down(steps) => {
                Position {
                    row: start_pos.row + steps,
                    col: start_pos.col,
                }
            },
            Self::Left(steps) => {
                Position {
                    row: start_pos.row,
                    col: start_pos.col - steps,
                }
            },
            Self::Right(steps) => {
                Position {
                    row: start_pos.row,
                    col: start_pos.col + steps,
                }
            },
        }
    }
}

fn colour_line_grid(grid: &mut Vec<Vec<bool>>, line: &Direction, pos: &Position) {
    let steps = match line {
        Direction::Up(steps) => *steps,
        Direction::Down(steps) => *steps,
        Direction::Left(steps) => *steps,
        Direction::Right(steps) => *steps,
    };

    for step in 1..steps + 1 {
        match line {
            Direction::Right(_) => {
                grid[pos.row][pos.col + step] = true;
            },
            Direction::Left(_) => {
                grid[pos.row][pos.col - step] = true;
            },
            Direction::Up(_) => {
                grid[pos.row - step][pos.col] = true;
            },
            Direction::Down(_) => {
                grid[pos.row + step][pos.col] = true;
            },
        };
    }
}

fn create_boundary_grid(instructions: &Vec<Direction>) -> Vec<Vec<bool>> {
    let mut grid = vec![];

    for r in 0..1000 {
        grid.push(vec![]);
        for _ in 0..1000 {
            grid[r].push(false);
        }
    }

    let mut position = Position {
        row: grid.len() / 2,
        col: grid[0].len() / 2,
    };
    grid[position.row][position.col] = true;

    for instruction in instructions {
        colour_line_grid(&mut grid, instruction, &position);
        position = instruction.apply_position(position);
    }

    grid
}

#[derive(Debug, PartialEq, Eq)]
enum ColourMode {
    Off,
    On,
}

fn colour_inside(grid: &mut Vec<Vec<bool>>) {
    let mut prev_row = &vec![];

    for (row_i, row) in grid.iter_mut().enumerate() {
        if row_i == 0 {
            continue;
        }
        let unaltered = row.clone();
        let mut colour_mode = ColourMode::Off;

        for col_i in 2..row.len() - 1 {
            if !unaltered[col_i - 2] && unaltered[col_i - 1] && !unaltered[col_i] {
                // FTF
                if colour_mode == ColourMode::Off {
                    colour_mode = ColourMode::On;
                }
                else if colour_mode == ColourMode::On {
                    colour_mode = ColourMode::Off;
                }
            }
            else if !unaltered[col_i] && unaltered[col_i - 1] && unaltered[col_i - 2] {
                // we're false and two behind us are true
                if prev_row[col_i] {
                    colour_mode = ColourMode::On;
                }
                else {
                    colour_mode = ColourMode::Off;
                }
            }
            if colour_mode == ColourMode::On {
                row[col_i] = true;
            }
        }
        prev_row = row;
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let input: (Vec<Direction>, Vec<&str>) = parse(input).into_iter().unzip();
    let input = input.0;
    let mut grid = create_boundary_grid(&input);
    
    colour_inside(&mut grid);

    let result: usize = grid.iter()
        .map(|row| row.iter().filter(|item| **item).count())
        .sum();
    
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    None
}

fn parse(input: &str) -> Vec<(Direction, &str)> {
    let input = input.split("\n").map(|line| line.split(" ").collect::<Vec<_>>());
    let mut parsed = vec![];

    for instruction in input {
        let steps: usize = instruction[1].parse().unwrap();
        let direction = match instruction[0] {
            "U" => Direction::Up(steps),
            "D" => Direction::Down(steps),
            "L" => Direction::Left(steps),
            "R" => Direction::Right(steps),
            _ => panic!("Unexpected direction"),
        };
        let colour = &instruction[2][2..8];

        parsed.push((direction, colour));
    }

    parsed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
