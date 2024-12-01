use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for (lhs, rhs) in input.lines().map(|l| {
        let (lhs, rhs) = l.split_once("   ").unwrap();
        (lhs.parse::<u32>().unwrap(), rhs.parse::<u32>().unwrap())
    }) {
        left.push(lhs);
        right.push(rhs);
    }

    left.sort();
    right.sort();
    let mut acc = 0;
    for (lhs, rhs) in left.iter_mut().zip(right.iter_mut()) {
        acc += lhs.abs_diff(*rhs);
    }
    Some(acc)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left = Vec::new();
    let mut right: HashMap<u32, u32> = HashMap::new();
    for (lhs, rhs) in input.lines().map(|l| {
        let (lhs, rhs) = l.split_once("   ").unwrap();
        (lhs.parse::<u32>().unwrap(), rhs.parse::<u32>().unwrap())
    }) {
        left.push(lhs);
        right.entry(rhs).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut sim_score = 0;
    for entry in left {
        let sim = right.get(&entry).unwrap_or(&0);
        sim_score += entry * sim;
    }

    Some(sim_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
