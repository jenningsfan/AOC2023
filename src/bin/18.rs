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
                    row: start_pos.row.saturating_sub(*steps),
                    col: start_pos.col,
                }
            },
            Self::Down(steps) => {
                Position {
                    row: start_pos.row + *steps,
                    col: start_pos.col,
                }
            },
            Self::Left(steps) => {
                Position {
                    row: start_pos.row,
                    col: start_pos.col.saturating_sub(*steps),
                }
            },
            Self::Right(steps) => {
                Position {
                    row: start_pos.row,
                    col: start_pos.col + *steps,
                }
            },
        }
    }
}

fn colour_col(grid: &mut Vec<Vec<bool>>, row: isize, col: usize) {
    if row >= 0 {
        let row = row as usize;
        if let Some(element) = grid.get_mut(row) {
            colour_row(element, col as isize);
        }
        else {
            grid.push(vec![]); // it should be fine like this
            colour_row(grid.last_mut().unwrap(), col as isize);
        }
    }
    else {
        grid.insert(0, vec![]); // it should be fine like this
        colour_row(&mut grid[0], col as isize);
    }
}

fn colour_row(row: &mut Vec<bool>, index: isize) {
    if row.len() == 0 && index == 0 {
        row.push(true);
        return;
    }
    
    if index >= 0 {
        let index = index as usize;
        if let Some(element) = row.get_mut(index) {
            *element = true;
        }
        else {
            let to_add = index - row.len();
            
            for _ in 0..to_add {
                row.push(false);
            }
            row.push(true);
        }
    }
    else {
        let index = -index - 1;

        for _ in 0..index {
            row.insert(0, false);
        }

        row.insert(0, true);
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
                colour_row(&mut grid[pos.row], (pos.col + step) as isize);
            },
            Direction::Left(_) => {
                colour_row(&mut grid[pos.row], pos.col as isize - step as isize);
            },
            Direction::Up(_) => {
                colour_col(grid, pos.row as isize - step as isize, pos.col);
            },
            Direction::Down(_) => {
                colour_col(grid, pos.row as isize + step as isize, pos.col);
            },
        };
    }
}

fn create_boundary_grid(instructions: &Vec<Direction>) -> Vec<Vec<bool>> {
    let mut grid = vec![vec![true]];
    let mut position = Position {
        row: 0,
        col: 0,
    };

    for instruction in instructions {
        colour_line_grid(&mut grid, instruction, &position);
        position = instruction.apply_position(position);
    }

    grid
}

fn colour_inside(grid: &mut Vec<Vec<bool>>) {
    for row in grid {
        let plan = row.clone();
        let mut filling = false;

        for i in 0..row.len() - 1{
            if plan[i] && !plan[i + 1] {
                filling = !filling;
            }
            if !row[i] && filling {
                row[i] = true;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let input: (Vec<Direction>, Vec<&str>) = parse(input).into_iter().unzip();
    let input = input.0;
    let mut grid = create_boundary_grid(&input);
    dbg!(&grid);
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
