advent_of_code::solution!(7);

fn is_valid(expected: u64, current: u64, rest: &[u64]) -> bool {
    if expected == current && rest.is_empty() {
        return true;
    }
    if current > expected {
        return false;
    }
    if rest.is_empty() {
        return false;
    }
    return is_valid(expected, current + rest[0], &rest[1..])
        || is_valid(expected, current * rest[0], &rest[1..]);
}

pub fn part_one(input: &str) -> Option<u64> {
    let operations = input.lines().map(|l| {
        let (result, rest) = l.split_once(": ").expect("2p");
        let ops = rest
            .split(' ')
            .map(|op| op.parse::<u64>().expect("u64"))
            .collect::<Vec<_>>();
        (result.parse::<u64>().expect("u64"), ops)
    });

    let mut sum = 0;
    for (expected, ops) in operations {
        if !is_valid(expected, ops[0], &ops[1..]) {
            continue;
        }
        sum += expected;
    }
    Some(sum)
}

fn is_valid_p2(expected: u64, current: u64, rest: &[u64]) -> bool {
    if expected == current && rest.is_empty() {
        return true;
    }
    if current > expected || rest.is_empty() {
        return false;
    }
    return is_valid_p2(expected, concat(current, rest[0]), &rest[1..])
        || is_valid_p2(expected, current + rest[0], &rest[1..])
        || is_valid_p2(expected, current * rest[0], &rest[1..]);
}

fn concat(lhs: u64, rhs: u64) -> u64 {
    let rhs_digits = ((rhs as f64).log10() + 1.0) as u32;
    return lhs * 10u64.pow(rhs_digits) + rhs;
}

pub fn part_two(input: &str) -> Option<u64> {
    let operations = input.lines().map(|l| {
        let (result, rest) = l.split_once(": ").expect("2p");
        let ops = rest
            .split(' ')
            .map(|op| op.parse::<u64>().expect("u64"))
            .collect::<Vec<_>>();
        (result.parse::<u64>().expect("u64"), ops)
    });

    let mut sum = 0;
    for (expected, ops) in operations {
        if !is_valid_p2(expected, ops[0], &ops[1..]) {
            continue;
        }
        sum += expected;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(874180, concat(874, 180));
        assert_eq!(1019, concat(10, 19));
        assert_eq!(9, concat(0, 9));
        assert_eq!(result, Some(11387));
    }
}
