use std::str::Chars;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.split("\n");
    let no_of_operands = lines.clone().count() - 1;

    let mut operands = lines.clone().take(no_of_operands).map(|line| line.split_whitespace()).collect::<Vec<_>>();
    let mut operators = lines.last().unwrap().split_whitespace();

    let mut sum = 0u64;
    loop {
        let operator = if let Some(operator) = operators.next() {
            if operator == "+" { |a, b| a + b } else { |a, b| a * b}
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
    let lines = input.split("\n");
    let no_of_operands = lines.clone().count() - 1;

    fn addr_of(s: &str) -> usize { s.as_ptr() as usize }
    fn split_whitespace_indices(s: &str) -> impl Iterator<Item = (usize, &str)> { s.split_whitespace().map(move |sub| (addr_of(sub) - addr_of(s), sub)) }

    let operands = lines.clone().into_iter().take(no_of_operands);
    let operators = split_whitespace_indices(lines.last().unwrap());

    let mut sum = 0u64;

    for (index, operator) in operators {
        let operator = if operator == "+" { |a: u64, b: u64| a + b } else { |a: u64, b: u64| a * b };
        let mut inputs: Vec<Chars> = operands.clone().map(|operand| operand[index..].chars()).collect();

        let mut numbers: Vec<u64> = vec![];
        loop {
            let mut digits: Vec<char> = vec![];
            for input in inputs.iter_mut() {
                digits.push(input.next().unwrap_or(' '));
            }

            digits.reverse();

            let filtered = digits.into_iter().filter(|digit| *digit != ' ');
            if filtered.clone().count() == 0 {
                break
            }

            numbers.push(
                filtered.enumerate().fold(0u64, |carry, (index, char)| {
                    let digit = char.to_string().parse::<u64>().unwrap();
                    carry + digit * 10u64.pow(index as u32)
                })
            );
        }

        let result = numbers.into_iter().reduce(operator).unwrap();
        sum += result;
    }

    Some(sum)
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
        assert_eq!(result, Some(3263827));
    }
}
