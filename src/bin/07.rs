use std::collections::HashSet;

advent_of_code::solution!(7);

type Point = (usize, usize);

pub fn part_one(input: &str) -> Option<u64> {
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
    None
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
        assert_eq!(result, None);
    }
}
