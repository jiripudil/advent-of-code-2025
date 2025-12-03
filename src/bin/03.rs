advent_of_code::solution!(3);

fn solve(input: &str, total_number_of_digits: usize) -> u64 {
    let lines = input.split("\n");
    let mut sum: u64 = 0;

    lines.for_each(|line| {
        let digits = line.chars().map(|c| c.to_digit(10).unwrap() as u64);
        let len = digits.clone().count();

        let mut values = vec![0u64; total_number_of_digits];

        let mut remaining_number_of_digits = total_number_of_digits;
        let mut offset = 0usize;
        while remaining_number_of_digits > 0 {
            let mut largest = 0u64;
            let mut lg_index = 0usize;

            let take = len - offset - remaining_number_of_digits + 1;
            for (i, digit) in digits.clone().skip(offset).take(take).enumerate() {
                if digit > largest {
                    largest = digit;
                    lg_index = i;
                }
            }

            values[remaining_number_of_digits - 1] = largest;
            remaining_number_of_digits -= 1;
            offset += lg_index + 1;
        }

        let joltage = values.iter().enumerate().fold(0u64, |carry, (index, value)| {
            carry + (value * 10u64.pow(index as u32))
        });

        sum += joltage;
    });

    sum
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 12))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
