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

fn is_anything_above(grid: &Vec<Vec<bool>>, row: usize, col: usize) -> bool {
    for i in 0..row {
        if grid[i][col] {
            return true;
        }
    }

    false
}

fn is_corner(grid: &Vec<Vec<bool>>, row: usize, col: usize) -> bool {
    (grid[row + 1][col] || grid[row - 1][col]) 
        && (grid[row][col + 1] || grid[row][col - 1])
        && grid[row][col]
}

fn should_switch_for_section(grid: &Vec<Vec<bool>>, row1: usize, col1: usize, row2: usize, col2: usize) -> bool{
    !(
        ((grid[row1 - 1][col1] && grid[row1][col1] && grid[row1][col1 + 1]) && (grid[row2 - 1][col2] && grid[row2][col2] && grid[row2][col2 - 1]))
        || ((grid[row1 + 1][col1] && grid[row1][col1] && grid[row1][col1 + 1]) && (grid[row2 + 1][col2] && grid[row2][col2] && grid[row2][col2 - 1]))
    )
}

fn colour_inside(grid: &mut Vec<Vec<bool>>) {
    let grid_plan = grid.clone();

    for (row_i, row) in grid.iter_mut().enumerate() {
        let plan = row.clone();
        let mut dug = row.clone();
        let mut filling = false;
        let mut switches = 0;
        let mut last_switch = 0;

        for col_i in 1..row.len() - 1 { 
            if plan[col_i] != plan[col_i + 1] {
                last_switch = col_i;
            }
            if plan[col_i] && !plan[col_i + 1] && should_switch_for_section(&grid_plan, row_i, last_switch, row_i, col_i) {
                filling = !filling;
                switches += 1;
                last_switch = col_i;
            }
            if !row[col_i] && filling   {
                dug[col_i] = true;
            }
        }

        if switches % 2 != 0 {
            for i in 0..last_switch {
                row[i] = dug[i];
            }
        }
        else {
            *row = dug;
        }
    }
}

fn print_grid(grid: &Vec<Vec<bool>>) {
    for row in grid {
        for col in row {
            match col {
                &true => print!("#"),
                &false => print!("."),
            };
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let input: (Vec<Direction>, Vec<&str>) = parse(input).into_iter().unzip();
    let input = input.0;
    let mut grid = create_boundary_grid(&input);
    print_grid(&grid);
    colour_inside(&mut grid);
    println!();
    print_grid(&grid);
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
