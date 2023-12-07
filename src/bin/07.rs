advent_of_code::solution!(7);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: Vec<u8>,
    bid: u32,
}

impl Hand {
    fn from_input(input: &str, part2: bool) -> Self {
        let input: Vec<&str> = input.split(" ").collect();
        let (cards, bid) = (input[0], input[1]);
        let bid = bid.parse().unwrap();

        let lookup = vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];
        let mut converted_cards: Vec<u8> = vec![];

        for char in cards.chars() {
            converted_cards.push(lookup.iter().position(|c| *c == char).unwrap() as u8);
        }

        let mut frequencies = vec![0; 13];

        for card in &converted_cards {
            frequencies[*card as usize] += 1;
        }

        // Sadly this doesn't work. was worth a try though
        if part2 {
            let jokers = frequencies[frequencies.len() - 1];
            frequencies.sort_by(|a, b| b.cmp(a));

            let mut hand_type: HandType = HandType::HighCard;

            if frequencies[0] + jokers >= 5 {
                hand_type = HandType::FiveOfAKind;
            }
            else if frequencies[0] + jokers >= 4 {
                hand_type = HandType::FourOfAKind;
            }
            else if frequencies[0] + jokers >= 3 {
                if frequencies[1] >= 2 {
                    hand_type = HandType::FullHouse;
                }
                else {
                    hand_type = HandType::ThreeOfAKind;
                }
            }
            else if frequencies[0] + jokers >= 2 && frequencies[1] == 2 {
                hand_type = HandType::TwoPair;
            }
            else if frequencies[0] + jokers >= 2 {
                hand_type = HandType::OnePair;
            }

            Self {
                cards: converted_cards,
                hand_type,
                bid,
            }
        }
        else {
            frequencies.sort_by(|a, b| b.cmp(a));
            let mut hand_type: HandType = HandType::HighCard;

            if frequencies[0] == 5 {
                hand_type = HandType::FiveOfAKind;
            }
            else if frequencies[0] == 4 {
                hand_type = HandType::FourOfAKind;
            }
            else if frequencies[0] == 3 {
                if frequencies[1] == 2 {
                    hand_type = HandType::FullHouse;
                }
                else {
                    hand_type = HandType::ThreeOfAKind;
                }
            }
            else if frequencies[0] == 2 && frequencies[1] == 2 {
                hand_type = HandType::TwoPair;
            }
            else if frequencies[0] == 2 {
                hand_type = HandType::OnePair;
            }

            Self {
                cards: converted_cards,
                hand_type,
                bid,
            }
        }
        

        

        
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut input = parse(input, false);
    input.sort_by(|a, b| b.cmp(a));

    let mut total = 0;
    for (rank, hand) in input.iter().enumerate() {
        let rank = rank as u32 + 1;
        total += rank * hand.bid;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input = parse(input, true);
    input.sort_by(|a, b| b.cmp(a));

    let mut total = 0;
    for (rank, hand) in input.iter().enumerate() {
        let rank = rank as u32 + 1;
        total += rank * hand.bid;
    }

    dbg!(Hand::from_input("QJKQ2 0", true));
    
    Some(total)
}

fn parse(input: &str, part2: bool) -> Vec<Hand> {
    let input = input.split("\n");
    let mut hands = vec![];

    for hand in input {
        hands.push(Hand::from_input(hand, part2));
    }

    hands
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
