advent_of_code::solution!(14);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Rock {
    Cube,
    Sphere,
    None
}

fn push_rocks(rocks: &mut Vec<Rock>) {
    for i in 0..rocks.len() {
        if rocks[i] == Rock::Sphere {
            for j in (0..i).rev() {
                if rocks[j] != Rock::None {
                    rocks[i] = Rock::None;
                    rocks[j + 1] = Rock::Sphere;
                    break;
                }
                else if j == 0 {
                    rocks[i] = Rock::None;
                    rocks[0] = Rock::Sphere;
                }
            }
        }
    }
}

fn load_of_rocks(rocks: &Vec<Vec<Rock>>) -> usize {
    let mut total = 0;

    for i in 0..rocks.len() {
        for j in 0..rocks[0].len() {
            if rocks[j][i] == Rock::Sphere {
                let distance = rocks.len() - i;
                total += distance;
            }
            
        }
    }

    total
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut input = parse(input);

    for row in input.iter_mut() {
        push_rocks(row);
    }
    
    Some(load_of_rocks(&input))
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    None
}

fn parse(input: &str) -> Vec<Vec<Rock>> {
    let input = input.split("\n").map(|line| line.chars());        
    let mut rocks = vec![];

    for _ in 0..input.clone().count() {
        rocks.push(vec![]);
    }

    for row in input {
        for (j, rock) in row.enumerate() {
            rocks[j].push(match rock {
                'O' => Rock::Sphere,
                '#' => Rock::Cube,
                '.' => Rock::None,
                unexpected => panic!("{unexpected} is not a type of rock"),
            });
        }
    }

    rocks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
