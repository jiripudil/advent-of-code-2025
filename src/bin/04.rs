use std::collections::HashMap;

advent_of_code::solution!(4);

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate { x: isize, y: isize }

fn parse_grid(input: &str) -> HashMap<Coordinate, bool> {
    let mut grid: HashMap<Coordinate, bool> = HashMap::new();
    for (y, line) in input.split('\n').enumerate() {
        for (x, char) in line.chars().enumerate() {
            let coordinate = Coordinate { x: x as isize, y: y as isize };
            grid.insert(coordinate, char == '@');
        }
    }

    grid
}

fn find_accessible_rolls(grid: &HashMap<Coordinate, bool>) -> Vec<&Coordinate> {
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

    accessible_rolls
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_grid(input);
    let accessible_rolls = find_accessible_rolls(&grid);
    Some(accessible_rolls.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse_grid(input);
    let mut removed_rolls = 0u64;
    loop {
        let current_grid = grid.clone();
        let accessible_rolls = find_accessible_rolls(&current_grid);
        if accessible_rolls.len() == 0 {
            break
        }

        removed_rolls += accessible_rolls.len() as u64;

        for accessible_roll in accessible_rolls {
            grid.remove(accessible_roll);
        }
    }

    Some(removed_rolls)
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
        assert_eq!(result, Some(43));
    }
}
