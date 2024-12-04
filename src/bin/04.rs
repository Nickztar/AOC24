use std::collections::HashMap;

advent_of_code::solution!(4);

type Pos = (i32, i32);

pub fn part_one(input: &str) -> Option<u32> {
    let word_map = input
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(move |(y, c)| ((x as i32, y as i32), c))
        })
        .collect::<HashMap<Pos, char>>();
    const DIRECTIONS: [Pos; 8] = [
        (0, 1),   // down
        (0, -1),  // up
        (1, 1),   // down right
        (-1, 1),  // down left
        (1, -1),  // up right
        (-1, -1), // up left
        (1, 0),   // right
        (-1, 0),  // left
    ];
    const MAS: [char; 3] = ['M', 'A', 'S'];
    let mut valid = 0;
    let x_positions = word_map.iter().filter(|(_, c)| c == &&'X');
    for (start_pos, _) in x_positions {
        'dir: for direction in DIRECTIONS {
            let mut current_pos = *start_pos;
            for target in MAS {
                match find_char(current_pos, target, direction, &word_map) {
                    Some(new_pos) => current_pos = new_pos,
                    _ => continue 'dir,
                }
            }
            valid += 1;
        }
    }
    Some(valid)
}

fn change_pos((x, y): Pos, (d_x, d_y): Pos) -> Pos {
    return (x + d_x, y + d_y);
}

fn find_char(pos: Pos, char: char, dir: Pos, map: &HashMap<Pos, char>) -> Option<Pos> {
    let new_pos = change_pos(pos, dir);
    _ = match map.get(&new_pos) {
        Some(c) if c == &char => c,
        _ => return None,
    };
    Some(new_pos)
}

pub fn part_two(input: &str) -> Option<u32> {
    let word_map = input
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(move |(y, c)| ((x as i32, y as i32), c))
        })
        .collect::<HashMap<Pos, char>>();
    const DIAG_DIRECTIONS: [Pos; 4] = [
        (1, 1),   // down right
        (-1, 1),  // down left
        (1, -1),  // up right
        (-1, -1), // up left
    ];

    const TARGETS: [char; 2] = ['A', 'S'];
    let mut middle_positions: HashMap<Pos, u32> = HashMap::new();
    let x_positions = word_map.iter().filter(|(_, c)| c == &&'M');
    for (start_pos, _) in x_positions {
        'dir: for direction in DIAG_DIRECTIONS {
            let mut current_pos = *start_pos;
            let mut a_position: Option<_> = None;
            for target in TARGETS {
                match find_char(current_pos, target, direction, &word_map) {
                    Some(new_pos) => current_pos = new_pos,
                    _ => continue 'dir,
                }
                if target == 'A' {
                    a_position = Some(current_pos);
                }
            }
            middle_positions
                .entry(a_position.unwrap())
                .and_modify(|f| *f += 1)
                .or_insert(1);
        }
    }
    Some(middle_positions.iter().filter(|(_, c)| c == &&2).count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
