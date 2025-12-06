use std::cmp;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut part = 1;
    let mut fresh_ingredients: Vec<(u64, u64)> = vec![];
    let mut no_of_fresh_ingredients = 0u64;

    input.split("\n").for_each(|line| {
        if line.is_empty() {
            part = 2;
            return
        }

        if part == 1 {
            let mut range = line.split("-");
            let start = range.next().unwrap().parse::<u64>().unwrap();
            let end_inclusive = range.next().unwrap().parse::<u64>().unwrap();
            fresh_ingredients.push((start, end_inclusive));
        }

        if part == 2 {
            let ingredient = line.parse::<u64>().unwrap();
            for (start, end_inclusive) in &fresh_ingredients {
                if ingredient >= *start && ingredient <= *end_inclusive {
                    no_of_fresh_ingredients += 1;
                    return
                }
            }
        }
    });

    Some(no_of_fresh_ingredients)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut fresh_ingredients: Vec<(u64, u64)> = vec![];

    for line in input.split("\n") {
        if line.is_empty() {
            break
        }

        let mut range = line.split("-");
        let start = range.next().unwrap().parse::<u64>().unwrap();
        let end_inclusive = range.next().unwrap().parse::<u64>().unwrap();
        fresh_ingredients.push((start, end_inclusive));
    }

    fresh_ingredients.sort_by(|(a, _), (b, _)| a.cmp(b));

    let mut result: Vec<(u64, u64)> = vec![];
    result.push(fresh_ingredients[0]);

    for i in 1..fresh_ingredients.len() {
        let current = fresh_ingredients[i];
        let j = result.len() - 1;

        if current.0 >= result[j].0 && current.0 <= result[j].1 {
            result[j].1 = cmp::max(current.1, result[j].1);
        } else {
            result.push(current);
        }
    }

    let no_of_fresh_ingredients = result.iter().fold(0, |acc, (start, end)| acc + (end - start + 1));
    Some(no_of_fresh_ingredients)
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
        assert_eq!(result, Some(14));
    }
}
