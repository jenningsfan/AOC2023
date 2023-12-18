use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{min, max};

advent_of_code::solution!(5);

lazy_static! {
    //static ref CATERGORY_REGEX: Regex = Regex::new(r"(\w+)-to-(\w+)").unwrap(); 
    static ref DIGITS_REGEX: Regex = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
}

#[derive(Debug, Clone, Copy)]
struct MapRange {
    source_start: i64,
    source_end: i64,
    destination_offset: i64
}

#[derive(Clone)]
struct CategoryMap {
    /*source: String,
    destination: String,*/
    map: Vec<MapRange>,
}

impl CategoryMap {
    fn from_input(input: &str) -> Self {
        let input: Vec<&str> = input.split('\n').filter(|&l| !l.is_empty()).collect();

        /*let category_names = input[0];
        let (_, [source, destination]) = CATERGORY_REGEX.captures(category_names)
            .expect(format!("{category_names} failed to parse").as_str()).extract();*/

        let mut map: Vec<MapRange> = vec![];

        for line in &input[1..] {
            let (_, [destination, source, range]) = DIGITS_REGEX.captures(*line).unwrap().extract();
            let destination: i64 = destination.parse().unwrap();
            let source: i64 = source.parse().unwrap();
            let range: i64 = range.parse().unwrap();

            map.push(MapRange {
                source_start: source,
                source_end: source + range,
                destination_offset: destination as i64 - source as i64,
            });
        }
        
        map.sort_by(|a, b| a.source_start.cmp(&b.source_start));
        
        Self {
            /*source: source.to_string(),
            destination: destination.to_string(),*/
            map,
        }
    }

    fn get_destination_number(&self, source: i64) -> i64 {
        for range in &self.map {
            if source >= range.source_start && source < range.source_end {
                return (source as i64 + range.destination_offset) as i64;
            }
        }
        
        source
    }

    fn get_destination_ranges_for(&self, source: &MapRange) -> Vec<MapRange> {
        let mut ranges = vec![];
        let mut last_end = None;
        let mut last_range: Option<MapRange> = None;

        for dest in &self.map {
            if dest.source_end < source.source_start || dest.source_start > source.source_end {
                continue;
            }

            // all the ranges we are dealing with MUST contain it
            
            if max(source.source_start, dest.source_start) + dest.destination_offset == 0 {
                //panic!("HERE");
            }

            ranges.push(MapRange {
                source_start: max(source.source_start, dest.source_start) + dest.destination_offset,
                source_end: min(source.source_end, dest.source_end) + dest.destination_offset,
                destination_offset: dest.destination_offset,
            });

            if last_end != Some(dest.source_start) && last_end.is_some() {
                if last_end.unwrap() == 0 {
                    //panic!("HERE");
                }

                ranges.push(MapRange {
                    source_start: last_end.unwrap(),
                    source_end: dest.source_start,
                    destination_offset: 0,
                });

                let ranges_last = ranges.len() - 1;
                ranges.swap(ranges_last, ranges_last - 1)
            }

            last_end = Some(max(source.source_start, dest.source_start));
            last_range = Some(*dest);
        }

        if ranges.len() == 0 {
            ranges.push(source.clone());
        }

        else {
        if ranges[0].source_end - ranges[0].destination_offset < source.source_end {
            ranges.push(MapRange {
                source_start: ranges[0].source_end,
                source_end: source.source_end,
                destination_offset: 0,
            });
        }
        else if ranges[0].source_start - ranges[0].destination_offset > source.source_start {
            ranges.push(MapRange {
                source_start: source.source_start,
                source_end: ranges[0].source_start,
                destination_offset: 0,
            });
        }

        for range in ranges.clone().windows(2) {
            if range[0].source_end - range[0].destination_offset < range[1].source_start - range[1].destination_offset {
                ranges.push(MapRange {
                    source_start: range[0].source_end,
                    source_end: range[1].source_start,
                    destination_offset: 0,
                });
            }
        }
    }
        

        // if ranges[ranges.len() - 1].source_end < source.source_end  {
        //     if ranges[ranges.len() - 1].source_end == 0 {
        //         //panic!("HERE");
        //     }

        //     ranges.push(MapRange {
        //         source_start: ranges[ranges.len() - 1].source_end,
        //         source_end: source.source_end,
        //         destination_offset: 0,
        //     });
        // }

        ranges
    }

    fn get_min_values_from_ranges(&self, remaining_maps: &Vec<CategoryMap>, ranges: &Vec<MapRange>) -> i64 {
        let mut remaining_maps = remaining_maps.clone();
        remaining_maps.remove(0);

        if remaining_maps.len() == 0 {
            let mut min_value = i64::MAX;

            for range in ranges {
                ////dbg!(range);
                ////dbg!(self.get_destination_ranges_for(range));
                min_value = min(min_value, self.get_destination_ranges_for(range).iter().map(|r| r.source_start).min().unwrap());
            }

            return min_value
        }
        else {
            let mut min_value = i64::MAX;

            for range in ranges {
                min_value = min(min_value, remaining_maps[0].get_min_values_from_ranges(&remaining_maps, &self.get_destination_ranges_for(range)));
            }

            return min_value
        }
    }

    // fn get_min_values_from_ranges(&self, remaining_maps: &Vec<CategoryMap>, ranges: &Vec<MapRange>) -> i64 {
    //     let mut remaining_maps = remaining_maps.clone();
    //     remaining_maps.remove(0);
    //     let ranges: Vec<Vec<MapRange>> = ranges.iter().map(|r| self.get_destination_ranges_for(&r)).collect();

    //     if remaining_maps.len() == 0 {
    //         let mut min_value = i64::MAX;

    //         for range in ranges {
    //             let min_from_this_range = range.iter().map(|r| r.source_start).min().unwrap();
    //             min_value = min(min_value, min_from_this_range);

    //             //min_value = min(min_value, //dbg!(range.source_start));
    //         }

    //         return min_value;
    //     }
    //     else {
    //         let mut min_value = i64::MAX;

    //         for range in ranges {
    //             let min_from_this_range = remaining_maps[0].get_min_values_from_ranges(&remaining_maps, &range);
    //             min_value = min(min_value, min_from_this_range);
    //         }

    //         return min_value;
    //     }
    //}
}

pub fn part_one(input: &str) -> Option<i64> {
    let (mut seeds, maps) = parse(input);

    for map in maps {
        seeds = seeds.iter().map(|s| map.get_destination_number(*s)).collect();
    }
    
    let result = *seeds.iter().min().unwrap();
    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    return None;
    
    let (seeds, maps) = parse(input);
    let mut seeds_range: Vec<MapRange> = vec![];

    ////dbg!(maps[maps.len() - 1].get_destination_ranges_for(&MapRange { source_start: 46, source_end: 47, destination_offset: 0 }));
    ////dbg!(maps[4].get_destination_ranges_for(&MapRange { source_start: 74, source_end: 88, destination_offset: 0 }));
    
    for i in (0..seeds.len()).step_by(2) {
        seeds_range.push(MapRange { 
            source_start: seeds[i],
            source_end: seeds[i] + seeds[i + 1], 
            destination_offset: 0,
        });
    }
    
    // let test_ranges = vec![
    //         MapRange {
    //             source_start: 1,
    //             source_end: 17,
    //             destination_offset: 0,
    //         },
    //         MapRange {
    //             source_start: 17,
    //             source_end: 29,
    //             destination_offset: 0,
    //         },
    //         MapRange {
    //             source_start: 29,
    //             source_end: 64,
    //             destination_offset: 0,
    //         },
    //         MapRange {
    //             source_start: 64,
    //             source_end: 70,
    //             destination_offset: 0,
    //         },
    //         MapRange {
    //             source_start: 70,
    //             source_end: 84,
    //             destination_offset: 0,
    //         },
    //         MapRange {
    //             source_start: 86,
    //             source_end: 88,
    //             destination_offset: 0,
    //         },
    //     ];

    //     let test_map = CategoryMap {map: test_ranges};
        
    //     //dbg!(test_map.get_destination_ranges_for(
    //         &MapRange{
    //             source_start: 62, 
    //             source_end: 89,
    //             destination_offset: 0
    //         }
    //     ));

    ////dbg!(&seeds_range);
    
    // let mut maps_test = maps.clone();
    // let test_seed = vec![MapRange { 
    //         source_start: 79,
    //         source_end: 93, 
    //         destination_offset: 0,
    // }];

    // for i in 0..maps.len() {
    //     //dbg!(maps[i].get_min_values_from_ranges(&maps_test, &test_seed));
    //     maps_test.remove(0);
    // }
    // panic!("stop");
    let result = maps[0].get_min_values_from_ranges(&maps, &seeds_range);
    // let result = maps[0].get_min_values_from_ranges(&maps, &vec![MapRange { 
    //     source_start: 79,
    //     source_end: 91, 
    //     destination_offset: 0,
    // }]);

    ////dbg!(maps[6].get_destination_ranges_for(&MapRange { source_start: 46, source_end: 57, destination_offset: 0 }));

    Some(result)
}

fn parse(input: &str) -> (Vec<i64>, Vec<CategoryMap>) {
    let mut input = input.split("\n");
    let seeds = input.next().unwrap()[7..].split(' ').map(|s| s.parse().unwrap()).collect();

    let mut groups: Vec<CategoryMap> = vec![];

    let mut buffer = "".to_string();
    for (i, line) in input.enumerate() {
        if line.contains("map") && i != 1 {
            groups.push(CategoryMap::from_input(buffer.as_str()));
            buffer = "".to_string();
        }

        buffer.push_str(line);
        buffer.push('\n');
    }

    groups.push(CategoryMap::from_input(buffer.as_str()));

    (seeds, groups)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
