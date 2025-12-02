advent_of_code::solution!(2);

fn is_invalid(n: u64, get_offsets: fn(u64) -> Vec<usize>) -> bool {
    let n_str = n.to_string();
    for offset in get_offsets(n) {
        if n_str.len() % offset != 0 {
            continue
        }

        let prefix = &n_str[0..offset];

        let mut cursor = offset;
        let mut found = true;
        while cursor < n_str.len() {
            if &n_str[cursor..cursor + offset] != prefix {
                found = false;
                break;
            }

            cursor += offset;
        }

        if found {
            return true
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0u64;
    let ranges = input.split(",");
    ranges.for_each(|range| {
        let mut pair = range.splitn(2, "-");
        let from = pair.next().unwrap().parse::<u64>().unwrap();
        let to = pair.next().unwrap().parse::<u64>().unwrap();

        for n in from..=to {
            let get_offset = |n: u64| -> Vec<usize> {
                if n.to_string().len() % 2 != 0 {
                    vec![]
                } else {
                    vec![n.to_string().len() / 2]
                }
            };

            if is_invalid(n, get_offset) {
                sum += n;
            }
        }
    });

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0u64;
    let ranges = input.split(",");
    ranges.for_each(|range| {
        let mut pair = range.splitn(2, "-");
        let from = pair.next().unwrap().parse::<u64>().unwrap();
        let to = pair.next().unwrap().parse::<u64>().unwrap();

        for n in from..=to {
            let get_offset = |n: u64| -> Vec<usize> {
                let threshold = n.to_string().len() / 2;
                (1..=threshold).collect()
            };

            if is_invalid(n, get_offset) {
                sum += n;
            }
        }
    });

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
