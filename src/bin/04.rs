use std::collections::HashMap;

advent_of_code::solution!(4);

#[derive(Eq, Hash, PartialEq)]
struct Coordinate { x: isize, y: isize }

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid: HashMap<Coordinate, bool> = HashMap::new();
    for (y, line) in input.split('\n').enumerate() {
        for (x, char) in line.chars().enumerate() {
            let coordinate = Coordinate { x: x as isize, y: y as isize };
            grid.insert(coordinate, char == '@');
        }
    }

    let mut accessible_rolls = vec![];

    for coordinate in grid.keys() {
        if !grid[coordinate] { continue; }

        let neighbours = vec![
            Coordinate { x: coordinate.x - 1, y: coordinate.y - 1 },
            Coordinate { x: coordinate.x - 1, y: coordinate.y },
            Coordinate { x: coordinate.x - 1, y: coordinate.y + 1 },
            Coordinate { x: coordinate.x, y: coordinate.y - 1 },
            Coordinate { x: coordinate.x, y: coordinate.y + 1 },
            Coordinate { x: coordinate.x + 1, y: coordinate.y - 1 },
            Coordinate { x: coordinate.x + 1, y: coordinate.y },
            Coordinate { x: coordinate.x + 1, y: coordinate.y + 1 },
        ];

        let mut neighbouring_rolls = 0;
        for neighbour in neighbours {
            if grid.contains_key(&neighbour) && grid[&neighbour] {
                neighbouring_rolls += 1;
            }
        }

        if neighbouring_rolls < 4 {
            accessible_rolls.push(coordinate);
        }
    }

    Some(accessible_rolls.len() as u64)
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
