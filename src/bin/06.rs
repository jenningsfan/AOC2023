advent_of_code::solution!(6);

#[derive(Clone, Copy, Debug)]
struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn ways_to_win(&self) -> u64 {
        let mut total = 0;

        for button_time in 0..self.time {
            if (self.time - button_time) * button_time > self.record {
                total += 1;
            }
        }

        total
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_part1(input);
    let races = input.iter().map(|race| race.ways_to_win());

    let mut result = 1;

    for race in races {
        result *= race;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_part2(input);

    Some(input.ways_to_win())
}

fn parse_part1(input: &str) -> Vec<Race> {
    let mut races = vec![];
    let mut input = input.split("\n")
        .map(|line| line.split(": ").nth(1).unwrap())
        .map(|line| line.split(' ').filter(|split| !split.is_empty()))
        .map(|line| line.map(|value| value.parse::<u64>().unwrap()));

    let input = input.next().unwrap().zip(input.next().unwrap());
    
    for pair in input {
        races.push(Race { 
            time: pair.0,
            record: pair.1
        });
    }

    races
}

fn parse_part2(input: &str) -> Race {
    let input: Vec<u64> = input.split("\n")
        .map(|line| line.split(": ").nth(1).unwrap())
        .map(|line| line.replace(' ', ""))
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    Race { 
        time: input[0],
        record: input[1],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
