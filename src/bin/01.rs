advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial: i64 = 50;
    let mut count = 0;

    input.lines().for_each(|line| {
        let direction = &line[0..1];
        let distance = line[1..].parse::<i64>().unwrap();
        dial += distance * match direction { "L" => -1, "R" => 1, _ => unreachable!() };

        if dial % 100 == 0 {
            count += 1;
        }
    });

    Some(count)
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
