use std::collections::{HashMap, HashSet};

advent_of_code::solution!(6);

type Position = (i32, i32);
#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .collect::<HashMap<Position, char>>();
    let (start_pos, _) = map
        .iter()
        .find(|(_, ch)| ch == &&'^')
        .expect("Has north start");
    let mut current_pos = *start_pos;
    let mut current_dir = Direction::North;
    let mut has_seen: HashSet<Position> = HashSet::new();
    'move_guard: loop {
        has_seen.insert(current_pos);
        let mut new_pos = move_pos(current_pos, get_delta(&current_dir));
        let mut should_turn = match map.get(&new_pos) {
            Some(c) if c == &'#' => true,
            None => break 'move_guard,
            _ => false,
        };

        while should_turn {
            // println!("{:?}, {}, {:?}", &new_pos, should_turn, &current_dir);
            current_dir = turn_dir(&current_dir);
            new_pos = move_pos(current_pos, get_delta(&current_dir));
            should_turn = match map.get(&new_pos) {
                Some(c) if c == &'#' => true,
                None => break 'move_guard,
                _ => false,
            };
        }
        current_pos = new_pos;
    }
    Some(has_seen.iter().count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn turn_dir(dir: &Direction) -> Direction {
    match dir {
        Direction::North => Direction::East,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
        Direction::East => Direction::South,
    }
}

fn get_delta(dir: &Direction) -> Position {
    match dir {
        Direction::North => (0, -1),
        Direction::South => (0, 1),
        Direction::West => (-1, 0),
        Direction::East => (1, 0),
    }
}

fn move_pos((x, y): Position, (d_x, d_y): Position) -> Position {
    return (x + d_x, y + d_y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
