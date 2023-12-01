advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split('\n');
    let mut total = 0;

    for line in lines {
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;
        
        for curr in line.chars() {
            let curr = curr.to_digit(10);

            if let Some(value) = curr {
                if first.is_none() {
                    first = Some(value);
                }
                last = Some(value);
            }
        }

        let first = first.unwrap_or(0);
        let last = last.unwrap_or(0);

        total += first * 10 + last;
        
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input = String::from(input);
    let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for num_outer in numbers {
        for num_inner in numbers {
            let outer_last = num_outer.chars().last().unwrap();
            let inner_first = num_inner.chars().next().unwrap();

            if outer_last == inner_first {
                let pair = num_outer.to_owned() + &num_inner[1..];
                let replacement = num_outer.to_owned() + &num_inner;

                input = input.replace(&pair, &replacement);
            }
        }
    }

    input = input.replace("one", "1");
    input = input.replace("two", "2");
    input = input.replace("three", "3");
    input = input.replace("four", "4");
    input = input.replace("five", "5");
    input = input.replace("six", "6");
    input = input.replace("seven", "7");
    input = input.replace("eight", "8");
    input = input.replace("nine", "9");

    let lines = input.split('\n');
    let mut total = 0;

    for line in lines {
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;
        
        for curr in line.chars() {
            let curr = curr.to_digit(10);

            if let Some(value) = curr {
                if first.is_none() {
                    first = Some(value);
                }
                last = Some(value);
            }
        }

        let first = first.unwrap_or(0);
        let last = last.unwrap_or(0);

        total += first * 10 + last;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
