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
