use std::cmp::max;
use regex::Regex;

advent_of_code::solution!(2);

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}

impl Hand {
    fn from_comma_list(list: &str) -> Self {
        let mut hand = Hand {
            red: 0,
            green: 0,
            blue: 0,
        };
        let list = list.split(", ");

        for cubes in list {
            let cubes: Vec<&str> = cubes.split(" ").collect();
            let num: u32 = cubes[0].parse().expect("Couldn't convert to number");
            let colour = cubes[1];

            if colour == "red" {
                hand.red = num;
            }
            else if colour == "green" {
                hand.green = num;
            }
            else if colour == "blue" {
                hand.blue = num;
            }
            else {
                panic!("{colour} is not a valid colour");
            }
        }

        hand
    }

    fn possible(self, max: &Self) -> bool {
        (self.red <= max.red) && (self.blue <= max.blue) && (self.green <= max.green)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let max = Hand {
        red: 12,
        green: 13,
        blue: 14,
    };

    let input = input.split("\n").enumerate();
    let mut total: u32 = 0;

    let game_matcher = Regex::new(r"(Game [0-9]*: )(.*)").unwrap();

    for (id, game) in input {
        let id: u32 = id as u32 + 1;
        let game = game_matcher.captures_iter(game);

        let mut rounds = vec![];
        for (_, [_, rounds_str]) in game.map(|c| c.extract()) {
            rounds = rounds_str.split("; ").collect();
        }
        
        let mut possible: bool = true;

        for round in rounds {
            let hand = Hand::from_comma_list(round);

            if !hand.possible(&max) {
                possible = false;
            }
        }
        
        if possible {
            total += id;
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.split("\n").enumerate();
    let game_matcher = Regex::new(r"(Game [0-9]*: )(.*)").unwrap();

    let mut total: u32 = 0;
    
    for (_, game) in input {
        let game = game_matcher.captures_iter(game);

        let mut rounds = vec![];
        for (_, [_, rounds_str]) in game.map(|c| c.extract()) {
            rounds = rounds_str.split("; ").collect();
        }
        
        let mut max_hand = Hand {
            red: 0,
            green: 0,
            blue: 0,
        };

        for round in rounds {
            let hand = Hand::from_comma_list(round);

            max_hand.red = max(hand.red, max_hand.red);
            max_hand.green = max(hand.green, max_hand.green);
            max_hand.blue = max(hand.blue, max_hand.blue);
        }

        total += max_hand.red * max_hand.blue * max_hand.green;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
