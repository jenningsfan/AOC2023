use std::collections::HashMap;

advent_of_code::solution!(20);

trait Module {
    fn send_pulse(&mut self, name: &str, pulse: bool);
}

struct FlipFlopModule<'a> {
    state: bool,
    name: &'a str,
    destination: Vec<&'a mut Box<dyn Module>>,
}

impl Module for FlipFlopModule<'_> {
    fn send_pulse(&mut self, name: &str, pulse: bool) {
        if !pulse {
            self.state = !self.state;
            self.destination.iter_mut()
                .for_each(|module| module.send_pulse(self.name, self.state));
        }
    }
}

struct ConjuctionModule<'a> {
    input_pulses: HashMap<&'a str, bool>,
    destination: Vec<&'a mut Box<dyn Module>>,
    name: &'a str,
}

impl Module for ConjuctionModule<'_> {
    fn send_pulse(&mut self, name: &str, pulse: bool) {
        let input = self.input_pulses.get_mut(name).unwrap();
        *input = pulse;

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
    name: &'a str,
}

impl Module for BroadcastModule<'_> {
    fn send_pulse(&mut self, name: &str, pulse: bool) {
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

fn parse(input: &str) -> Vec<&str> {
    input.split("\n").collect()
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
