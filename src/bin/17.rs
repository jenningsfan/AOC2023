use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::rc::Rc;

advent_of_code::solution!(17);

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Node {
    cost: u8,
    row: isize,
    col: isize,
    parent: Option<Rc<Node>>,
    parent_direction: Option<Direction>,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl Node {
    fn total_cost(&self) -> u32 {
        if self.parent.is_none() {
            return self.cost as u32;
        } else {
            return self.cost as u32 + self.parent.as_ref().unwrap().total_cost();
        }
    }

    fn directions_to_exclude(&self) -> Vec<Direction> {
        let mut directions = vec![];

        if let Some(parent_direction) = self.parent_direction {
            directions.push(parent_direction);

            // let mut prev_directions = vec![];
            // let mut curr_node = Some(self);

            // for _ in 0..3 {
            //     if let Some(node) = curr_node {
            //         if node.parent.is_some() {
            //             prev_directions.push(node.parent_direction.unwrap());
            //             curr_node = node.parent.as_deref();
            //         }
            //         else {
            //             break;
            //         }
            //     } else {
            //         break;
            //     }
            // }

            // if prev_directions.len() == 3 && prev_directions.windows(2).all(|w| w[0] == w[1]) {
            //     directions.push(prev_directions[0]);
            // }
        }

        directions
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn apply_movement(r: isize, c: isize, direction: Direction) -> (isize, isize) {
    let r = r as isize + match direction {
        Direction::Up => -1,
        Direction::Down => 1,
        _ => 0,
    };

    let c = c as isize + match direction {
        Direction::Left => -1,
        Direction::Right => 1,
        _ => 0,
    };

    (r, c)
}

fn get_neighbours_of_node(
    map: &Vec<Vec<u8>>,
    r: isize,
    c: isize,
    excluded_nodes: Vec<Direction>,
) -> Vec<((isize, isize), Direction)> {
    let mut neighbours = vec![];

    neighbours.push((apply_movement(r, c, Direction::Left), Direction::Left));
    neighbours.push((apply_movement(r, c, Direction::Right), Direction::Right));
    neighbours.push((apply_movement(r, c, Direction::Up), Direction::Up));
    neighbours.push((apply_movement(r, c, Direction::Down), Direction::Down));

    neighbours
        .into_iter()
        .filter(|node| {
            !excluded_nodes.contains(&node.1)
                && node.0 .0 >= 0
                && node.0 .0 < map.len() as isize
                && node.0 .1 >= 0
                && node.0 .1 < map[0].len() as isize
        })
        .collect()
}

fn manhattan(r1: isize, c1: isize, r2: isize, c2: isize) -> usize {
    (r1 - r2).abs() as usize + (c1 - c2).abs() as usize
}

fn astar(map: &Vec<Vec<u8>>, start_r: isize, start_c: isize, end_r: isize, end_c: isize) -> u32 {
    dbg!("hello\n");
    let mut open = BinaryHeap::new();
    let mut closed = HashSet::new();
    dbg!("hello");
    open.push(Rc::new(Node {
        cost: map[start_r as usize][start_c as usize],
        row: start_r,
        col: start_c,
        parent: None,
        parent_direction: None,
    }));

    loop {
        let current = open.pop().expect("Open should not be empty");
        dbg!(&current.cost);
        let row = current.row;
        let col = current.col;
        closed.insert(Rc::clone(&current));

        if row == end_r && col == end_c {
            return current.total_cost();
        }

        let neighbours = get_neighbours_of_node(map, row, col, current.directions_to_exclude());

        for (neighbour, direction) in neighbours {
            let score =
                current.total_cost() + manhattan(neighbour.0 as isize, neighbour.1 as isize, row, col) as u32 + map[row as usize][col as usize] as u32;
            let neighbour_node = Rc::new(Node {
                cost: score as u8,
                row: neighbour.0,
                col: neighbour.1,
                parent: Some(Rc::clone(&current)),
                parent_direction: Some(direction),
            });

            let mut item = open.clone();
            item.retain(|item| item.row == neighbour.0.try_into().unwrap() && item.col == neighbour.1.try_into().unwrap());
            let mut item_score_higher = item.clone();
            item_score_higher.retain(|node| (node.cost as u32) < score);

            if item.len() == 0 || item_score_higher.len() > 0 {
                open.retain(|node| !(node.row == neighbour.0 && node.col == neighbour.1));
            }

            open.push(neighbour_node);
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    return None;
    
    let input = parse(input);
    let result = astar(&input, 0, 0, input.len() as isize - 1, input[0].len() as isize - 1);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    // Implement the rest of part_two
    None
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .split("\n")
        .map(|line| line.chars().map(|block| block.to_digit(10).unwrap() as u8).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
