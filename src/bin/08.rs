use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(8);

#[derive(Debug, Clone)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn next_node<'a>(&'a self, direction: char, nodes: &'a HashMap<String, Node>) -> &Node {
        if direction == 'R' {
            return nodes.get(&self.right).unwrap();
        }
        else if direction == 'L' {
            return nodes.get(&self.left).unwrap();
        }
        else {
            panic!("Direction is {direction} not R or L");
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (directions, nodes) = parse(input);

    let mut current = nodes.get("AAA").unwrap();
    let mut nodes_visited = 0;

    for direction in directions.chars().cycle() {
        nodes_visited += 1;

        current = current.next_node(direction, &nodes);

        if current.name == "ZZZ" {
            break;
        }
    }

    Some(nodes_visited)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (directions, nodes) = parse(input);

    let mut current: Vec<&Node> = nodes.values().filter(|node| node.name.chars().last().unwrap() == 'A').collect();
    let mut nodes_visited = 0;
    dbg!(&current);
    for direction in directions.chars().cycle() {
        nodes_visited += 1;

        current = current.iter().map(|n| n.next_node(direction, &nodes)).collect();

        if current.iter().all(|node| node.name.chars().last().unwrap() == 'Z') {
            break;
        }
    }

    Some(nodes_visited)
}

fn parse(input: &str) -> (&str, HashMap<String, Node>) {
    let mut input = input.split("\n");
    let directions = input.next().unwrap();
    input.next();

    let mut nodes: HashMap<String, Node> = HashMap::new();
    let node_regex = Regex::new(r"(\w\w\w)+").unwrap();

    for line in input.clone() {
        let node_names: Vec<String> = node_regex.find_iter(line).map(|c| c.as_str().to_string()).collect();

        nodes.insert(node_names[0].clone(), Node {
            name: node_names[0].clone(),
            left: node_names[1].clone(),
            right: node_names[2].clone(),
        });
    }
    
    (directions, nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(6));
    }
}
