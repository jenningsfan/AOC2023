use std::collections::HashSet;

advent_of_code::solution!(4);

#[derive(Debug, Clone)]
struct Scratchcard {
    winning: HashSet<u32>,
    on_card: HashSet<u32>,
}

impl Scratchcard {
    fn from_table_row(row: &str) -> Self {
        let row: Vec<HashSet<u32>> = row
            .split(": ").nth(1).expect("There should be a value after the card number").split(" | ")
            .map(
                |row| row.split(" ").filter(|value| *value != "")
                .map(
                    |value| value.parse().expect(format!("{value} cannot be converted to an int").as_str())
                ).collect()
            ).collect();

        Self {
            winning: row[0].clone(),
            on_card: row[1].clone(),
        }
    }

    fn matches(&self) -> HashSet<u32> {
        self.on_card.intersection(&self.winning).map(|i| *i).collect()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);
    let mut total = 0;
    
    for card in input {
        let num_matches: u32 = card.matches().len() as u32;

        if num_matches > 0 {
            total += 2_u32.pow(num_matches - 1);
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input: Vec<(u32, Scratchcard)> = parse(input).iter().map(|row| (1, row.clone())).collect();
    let mut total = 0;

    for i in 0..input.len() {
        let (copies, card) = &input[i].clone();
        total += copies;
        
        let matches = card.matches().len() as u32;
        let new_cards = i + 1..i + matches as usize + 1;

        for new_card in new_cards {
            input[new_card].0 += copies;
        }
    }

    Some(total)
}

fn parse(input: &str) -> Vec<Scratchcard> {
    input.split("\n").map(Scratchcard::from_table_row).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
