use regex::Regex;

advent_of_code::solution!(15);

#[derive(Debug, Clone)]
struct Box {
    lenses: Vec<Lens>
}

#[derive(Debug, Clone)]
struct Lens {
    focal_length: u32,
    label: String,
}

fn perform_operation(boxes: &mut Vec<Box>, operation: &str) {
    let regex = Regex::new(r"([a-z]+)([=-])([0-9]+)*").unwrap();
    let matches = regex.captures(operation).unwrap();

    let label = &matches[1];
    let op_type = &matches[2];
    let box_id = calculate_hash(label) as usize;
    let light_box = &mut boxes[box_id];

    if op_type == "=" {
        let focal_length: u32 = (&matches[3]).parse().unwrap();
        let mut found_lens = false;

        for lens in light_box.lenses.iter_mut() {
            if lens.label == label {
                lens.focal_length = focal_length;
                found_lens = true;
                break;
            }
        }

        if !found_lens {
            let label = label.to_owned();

            light_box.lenses.push(
                Lens {
                    focal_length,
                    label,
                }
            );
        }
    }
    else if op_type == "-" {
        light_box.lenses = light_box.lenses.clone().into_iter().filter(|lens| lens.label != label).collect();
    }
    else {
        panic!("{} is not an operation type", op_type);
    }
}

fn calculate_hash(string: &str) -> u8 {
    let mut current: u16 = 0;

    for char in string.chars() {
        let char = char as u8;
        current += char as u16;
        current *= 17;
        current %= 256;
    }

    current as u8
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);
    let total = input.iter().map(|step| calculate_hash(*step) as u32).sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);
    let mut boxes = vec![];

    for _ in 0..256 {
        boxes.push(
            Box {
                lenses: vec![]
            }
        );
    }

    for operation in input{
        perform_operation(&mut boxes, operation);
    }

    let mut total = 0;

    for (box_i, light_box) in boxes.iter().enumerate() {
        for (lens_i, lens) in light_box.lenses.iter().enumerate() {
            total += (box_i + 1) * (lens_i + 1) * lens.focal_length as usize;
        }
    }

    Some(total)
}

fn parse(input: &str) -> Vec<&str> {
    input.split(",").collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
