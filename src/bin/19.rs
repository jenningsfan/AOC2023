use std::collections::HashMap;
use std::iter::zip;
use regex::Regex;

advent_of_code::solution!(19);

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    fallback: String,
}

impl Workflow {
    fn from_input(input: &str) -> Self {
        let input: Vec<&str> = input.split("{").collect();
        let name = input[0].to_string();
        let rules: Vec<&str> = input[1][..input[1].len() - 1].split(",").collect();
        let fallback = rules[rules.len() - 1].to_string();
        let rules = rules[..rules.len() - 1].iter();
        let rules = rules.map(|rule| Rule::from_input(*rule)).collect();

        Self {
            name,
            rules,
            fallback,
        }
    }

    fn next_workflow_for(&self, part: &HashMap<String, u32>) -> String {
        for rule in &self.rules {
            if rule.check_against_part(part) {
                return rule.workflow.clone();
            }
        }

        return self.fallback.clone();
    }
}

#[derive(Debug, Clone)]
struct Rule {
    category: String,
    operation: Operation,
    value: u32,
    workflow: String,
}

impl Rule {
    fn from_input(input: &str) -> Self {
        let regex = Regex::new(r"([a-z]+)([<>])(\d+):([a-zAR]+)").unwrap();
        let matches = regex.captures(input).unwrap();

        let category = matches[1].to_string();
        let operation = match &matches[2] {
            "<" => Operation::LessThan,
            ">" => Operation::GreaterThan,
            other => panic!("{other} is not a supported operation"),
        };
        let value = matches[3].parse().unwrap();
        let workflow = matches[4].to_string();

        Self {
            category,
            operation,
            value,
            workflow,
        }
    }

    fn check_against_part(&self, part: &HashMap<String, u32>) -> bool {
        let part_val = part.get(&self.category).unwrap();
        match self.operation {
            Operation::LessThan => {*part_val < self.value},
            Operation::GreaterThan => {*part_val > self.value},
        }
    }
}


#[derive(Debug, Clone)]
enum Operation {
    LessThan,
    GreaterThan,
}

fn is_accepted(part: &HashMap<String, u32>, workflows: &HashMap<String, Workflow>) -> bool {
    let mut workflow_name = "in".to_string();

    while workflow_name != "R".to_string() && workflow_name != "A".to_string() {
        let workflow = workflows.get(&workflow_name).unwrap();
        workflow_name = workflow.next_workflow_for(part);
    }

    workflow_name == "A".to_string()
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);
    let workflows = input.0;
    let parts = input.1;
    let result: u32 = parts.iter()
        .filter(|part| is_accepted(part, &workflows))
        .map(|part| part.values().sum::<u32>())
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    None
}

fn parse(input: &str) -> (HashMap<String, Workflow>, Vec<HashMap<String, u32>>) {
    let input: Vec<&str> = input.split("\n\n").collect();

    let workflows = input[0].split("\n").map(Workflow::from_input);
    let workflows_names = workflows.clone().map(|workflow| workflow.name);
    let mut workflows_map = HashMap::new();

    for (name, workflow) in zip(workflows_names, workflows) {
        workflows_map.insert(name, workflow);
    }

    

    let parts = input[1].split("\n");
    let mut parts_map = vec![];
    
    for part in parts {
        let part: Vec<(&str, u32)> = part[1..part.len() - 1]
            .split(",").map(|cat| cat.split("=").collect())
            .map(|cat: Vec<&str>| (cat[0], cat[1].parse().unwrap()))
            .collect();

        let mut part_map = HashMap::new();
        for (name, value) in part {
            part_map.insert(name.to_string(), value);
        }

        parts_map.push(part_map);
    }

    (workflows_map, parts_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
