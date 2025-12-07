use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

type Point = (usize, usize);

fn parse_grid(input: &str) -> (usize, Point, Vec<Point>) {
    let mut start: Point = (0, 0);
    let mut splitters: Vec<Point> = vec![];
    let rows = input.lines().count();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == 'S' {
                start = (x, y);
            } else if char == '^' {
                splitters.push((x, y));
            }
        }
    }

    (rows, start, splitters)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (rows, start, splitters) = parse_grid(input);

    let mut beams = vec![start];
    let mut visited = vec![start];
    let mut splits = HashSet::new();
    while let Some(beam) = beams.pop() {
        let mut down = (beam.0, beam.1 + 1);
        loop {
            if down.1 >= rows {
                break;
            }

            if splitters.contains(&down) {
                splits.insert(down);
                for split in [(down.0 - 1, down.1), (down.0 + 1, down.1)] {
                    if !visited.contains(&split) {
                        visited.push(split);
                        beams.push(split);
                    }
                }

                break;
            }

            down = (down.0, down.1 + 1);
        }
    }

    Some(splits.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (rows, start, mut splitters) = parse_grid(input);
    splitters.reverse();

    let mut splitter_timelines: HashMap<Point, u64> = HashMap::with_capacity(splitters.len());
    for splitter in splitters.iter() {
        let mut timelines = 0u64;

        for split in [(splitter.0 - 1, splitter.1), (splitter.0 + 1, splitter.1)] {
            let mut down = (split.0, split.1 + 1);
            loop {
                if down.1 >= rows {
                    timelines += 1;
                    break;
                }

                if splitter_timelines.contains_key(&down) {
                    timelines += splitter_timelines[&down];
                    break;
                }

                down = (down.0, down.1 + 1);
            }
        }

        splitter_timelines.insert(*splitter, timelines);
    }

    let topmost_splitter = splitters.last().unwrap();
    let timelines = splitter_timelines[topmost_splitter];

    Some(timelines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
