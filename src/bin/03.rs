use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"(mul\((?<dig1>[0-9]+),(?<dig2>[0-9]+)\))").unwrap();
    let mut acc = 0;
    for (_, [_, dig1, dig2]) in re.captures_iter(input).map(|c| c.extract()) {
        let (lhs, rhs) = (dig1.parse::<u32>().unwrap(), dig2.parse::<u32>().unwrap());
        acc += lhs * rhs;
    }
    Some(acc)
}

pub fn part_two(input: &str) -> Option<u32> {
    let do_re = Regex::new(r"(do\(\))").unwrap();
    let dont_re = Regex::new(r"(don't\(\))").unwrap();
    let mul_re = Regex::new(r"(mul\((?<dig1>[0-9]+),(?<dig2>[0-9]+)\))").unwrap();
    let do_matches: Vec<_> = do_re.find_iter(input).map(|m| m.end()).collect();
    let dont_matches: Vec<_> = dont_re.find_iter(input).map(|m| m.end()).collect();
    let mut acc = 0;
    for (start_idx, (_, [_, dig1, dig2])) in mul_re
        .captures_iter(input)
        .map(|c| (c.get(0).unwrap().start(), c.extract()))
    {
        let prev_do = do_matches
            .iter()
            .filter(|end| end <= &&start_idx)
            .nth_back(0);
        let prev_dont = dont_matches
            .iter()
            .filter(|end| end <= &&start_idx)
            .nth_back(0);
        let should_do = match (prev_do, prev_dont) {
            (Some(do_idx), Some(dont_idx)) => do_idx > dont_idx,
            (Some(_), None) => true,
            (None, Some(_)) => false,
            _ => true,
        };
        if should_do {
            let (lhs, rhs) = (dig1.parse::<u32>().unwrap(), dig2.parse::<u32>().unwrap());
            acc += lhs * rhs;
        }
    }
    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
