use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(20);

trait Module {
    fn send_pulse(&mut self, name: &str, pulse: bool);
}

struct FlipFlopModule<'a> {
    state: bool,
    total_high: &'a mut u32,
    total_low: &'a mut u32,
    name: &'a str,
    destination: Vec<&'a mut Box<dyn Module>>,
}

impl Module for FlipFlopModule<'_> {
    fn send_pulse(&mut self, name: &str, pulse: bool) {
        if !pulse {
            *self.total_low += 1;
            self.state = !self.state;
            self.destination.iter_mut()
                .for_each(|module| module.send_pulse(self.name, self.state));
        }
        else {
            *self.total_high += 1;
        }
    }
}

struct ConjuctionModule<'a> {
    input_pulses: HashMap<String, bool>,
    total_high: &'a mut u32,
    total_low: &'a mut u32,
    destination: Vec<&'a mut Box<dyn Module>>,
    name: &'a str,
}

impl Module for ConjuctionModule<'_> {
    fn send_pulse(&mut self, name: &str, pulse: bool) {
        if pulse {
            *self.total_high += 1;
        }
        else {
            *self.total_low += 1;
        }

        let input = self.input_pulses.entry(name.to_string());
        *input.or_default() = pulse;

        if self.input_pulses.values().all(|pulse| *pulse) {
            self.destination.iter_mut()
                .for_each(|module| module.send_pulse(self.name, false));
        }
        else {
            self.destination.iter_mut()
                .for_each(|module| module.send_pulse(self.name, true));
        }
    }
}

struct BroadcastModule<'a> {
    destination: Vec<&'a mut Box<dyn Module>>,
    total_high: &'a mut u32,
    total_low: &'a mut u32,
    name: &'a str,
}

impl Module for BroadcastModule<'_> {
    fn send_pulse(&mut self, name: &str, pulse: bool) {
        if pulse {
            *self.total_high += 1;
        }
        else {
            *self.total_low += 1;
        }

        self.destination.iter_mut()
            .for_each(|module| module.send_pulse(self.name, pulse));
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    None
}

fn parse(input: &str) {
    let mut total_high = 0;
    let mut total_low = 0;

    let input = input.split("\n")
        .map(|module| module.split(" -> ").collect_tuple::<(&str, &str)>().unwrap())
        .map(|module| (module.0, module.1.split(", ").collect_vec()));

    let modules: HashMap<&str, Box<dyn Module>> = HashMap::new();

    for module in input {
        let name = module.0;
        let mod_type = name.chars().nth(0).unwrap();

        match mod_type {
            'b' => {},
            '%' => {},
            '&' => {},
            unexpected => panic!("{unexpected} is not valid"),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
