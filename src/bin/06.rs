advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.split("\n");
    let no_of_operands = lines.clone().count() - 1;

    let mut operands = lines.clone().take(no_of_operands).map(|line| line.split_whitespace()).collect::<Vec<_>>();
    let mut operators = lines.last().unwrap().split_whitespace();

    let mut sum = 0u64;
    loop {
        let operator = if let Some(operator) = operators.next() {
            if (operator == "+") { |a, b| a + b } else { |a, b| a * b}
        } else {
            break
        };

        let mut numbers = vec![];
        for operand in operands.iter_mut() {
            numbers.push(operand.next().unwrap().parse::<u64>().unwrap());
        }

        let result = numbers.into_iter().reduce(operator).unwrap();
        sum += result;
    }

    Some(sum)
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
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
