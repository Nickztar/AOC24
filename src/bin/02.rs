use std::collections::btree_map::Values;

advent_of_code::solution!(2);

#[derive(Default, PartialEq, Debug)]
enum Direction {
    #[default]
    Unknown,
    Descending,
    Ascending,
}
#[derive(Default, Debug)]
struct State {
    previous: Option<u32>,
    dir: Direction,
    is_unsafe: bool,
    unsafe_count: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let safe_routes = input
        .lines()
        .filter(|l| {
            !l.split_ascii_whitespace()
                .map(|p| p.parse::<u32>().unwrap())
                .fold(State::default(), |mut state, value| {
                    match state.previous {
                        Some(prev) => {
                            let is_asc = prev < value;
                            let diff = prev.abs_diff(value);
                            if diff < 1 || diff > 3 || state.is_unsafe {
                                state.is_unsafe = true;
                                return state;
                            }
                            match state.dir {
                                Direction::Unknown => {
                                    state.dir = if is_asc {
                                        Direction::Ascending
                                    } else {
                                        Direction::Descending
                                    };
                                }
                                Direction::Descending => {
                                    if is_asc {
                                        state.is_unsafe = true;
                                    }
                                }
                                Direction::Ascending => {
                                    if !is_asc {
                                        state.is_unsafe = true;
                                    }
                                }
                            };
                            state.previous = Some(value)
                        }
                        None => state.previous = Some(value),
                    };
                    state
                })
                .is_unsafe
        })
        .count();
    Some(safe_routes as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let safe_routes = input
        .lines()
        .filter(|l| {
            let levels = l
                .split_ascii_whitespace()
                .map(|p| p.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            for exclude_idx in 0..levels.len() {
                let state = levels
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| i != &exclude_idx)
                    .fold(State::default(), |mut state, (_, value)| {
                        match state.previous {
                            Some(prev) => {
                                let is_asc = prev < *value;
                                let diff = prev.abs_diff(*value);
                                if diff < 1 || diff > 3 || state.is_unsafe {
                                    state.is_unsafe = true;
                                    return state;
                                }
                                match state.dir {
                                    Direction::Unknown => {
                                        state.dir = if is_asc {
                                            Direction::Ascending
                                        } else {
                                            Direction::Descending
                                        };
                                    }
                                    Direction::Descending => {
                                        if is_asc {
                                            state.is_unsafe = true;
                                        }
                                    }
                                    Direction::Ascending => {
                                        if !is_asc {
                                            state.is_unsafe = true;
                                        }
                                    }
                                };
                                state.previous = Some(*value)
                            }
                            None => state.previous = Some(*value),
                        };
                        state
                    });
                if !state.is_unsafe && state.unsafe_count <= 1 {
                    return true;
                }
            }
            return false;
        })
        .count();
    Some(safe_routes as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
}
