use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (order_rules, updates) = input.split_once("\n\n").expect("Two parts");
    let mut left_map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut right_map: HashMap<u32, Vec<u32>> = HashMap::new();
    for (lhs, rhs) in order_rules.lines().map(|line| {
        line.split_once('|')
            .map(|(lhs, rhs)| (lhs.parse::<u32>().unwrap(), rhs.parse::<u32>().unwrap()))
            .unwrap()
    }) {
        left_map
            .entry(lhs)
            .and_modify(|rights| rights.push(rhs))
            .or_insert(vec![rhs]);
        right_map
            .entry(rhs)
            .and_modify(|lefts| lefts.push(lhs))
            .or_insert(vec![lhs]);
    }

    let default_vec: Vec<u32> = Vec::new();
    let mut acc = 0;
    'update: for update in updates.lines().map(|l| {
        l.split(',')
            .map(|c| c.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    }) {
        for (i, page) in update.iter().enumerate() {
            let come_after = left_map.get(page).unwrap_or(&default_vec);
            let come_before = right_map.get(page).unwrap_or(&default_vec);
            let rem_pages = &update[i + 1..update.len()];
            let prev_pages = &update[0..i];
            // dbg!(
            //     "Page: ",
            //     &page,
            //     " : ",
            //     &come_after,
            //     &come_before,
            //     &rem_pages,
            //     &prev_pages
            // );
            if rem_pages.iter().any(|page| come_before.contains(page)) {
                continue 'update;
            }

            if prev_pages.iter().any(|page| come_after.contains(page)) {
                continue 'update;
            }
        }
        let middle = update[((update.len() - 1) as f32 / 2.0).round() as usize];
        acc += middle;
    }
    Some(acc)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (order_rules, updates) = input.split_once("\n\n").expect("Two parts");
    let mut left_map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut right_map: HashMap<u32, Vec<u32>> = HashMap::new();
    for (lhs, rhs) in order_rules.lines().map(|line| {
        line.split_once('|')
            .map(|(lhs, rhs)| (lhs.parse::<u32>().unwrap(), rhs.parse::<u32>().unwrap()))
            .unwrap()
    }) {
        left_map
            .entry(lhs)
            .and_modify(|rights| rights.push(rhs))
            .or_insert(vec![rhs]);
        right_map
            .entry(rhs)
            .and_modify(|lefts| lefts.push(lhs))
            .or_insert(vec![lhs]);
    }

    let default_vec: Vec<u32> = Vec::new();
    let mut acc = 0;
    for update in updates.lines().map(|l| {
        l.split(',')
            .map(|c| c.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    }) {
        let mut to_check = update.clone();
        while let Some((incorrect_page, left)) =
            check_correct(&to_check, &left_map, &default_vec, &right_map)
        {
            let dir: i32 = if left { 1 } else { -1 };
            let mut new_idx = (incorrect_page as i32 + dir) as usize;
            to_check.swap(incorrect_page, new_idx);
            while let Some(_) = specific_correct(
                new_idx,
                &to_check,
                &left_map,
                &default_vec,
                &right_map,
                left,
            ) {
                let new_new_idx = (new_idx as i32 + dir) as usize;
                to_check.swap(new_idx, new_new_idx);
                new_idx = new_new_idx;
            }
        }
        if to_check == update {
            continue;
        }
        let middle = to_check[((to_check.len() - 1) as f32 / 2.0).round() as usize];
        acc += middle;
    }
    Some(acc)
}

fn check_correct(
    update: &Vec<u32>,
    left_map: &HashMap<u32, Vec<u32>>,
    default_vec: &Vec<u32>,
    right_map: &HashMap<u32, Vec<u32>>,
) -> Option<(usize, bool)> {
    for (i, page) in update.iter().enumerate() {
        let come_after = left_map.get(page).unwrap_or(default_vec);
        let come_before = right_map.get(page).unwrap_or(default_vec);
        let rem_pages = &update[i + 1..update.len()];
        let prev_pages = &update[0..i];
        if prev_pages.iter().any(|page| come_after.contains(page)) {
            return Some((i, false));
        }

        if rem_pages.iter().any(|page| come_before.contains(page)) {
            return Some((i, true));
        }
    }
    None
}

fn specific_correct(
    idx: usize,
    update: &Vec<u32>,
    left_map: &HashMap<u32, Vec<u32>>,
    default_vec: &Vec<u32>,
    right_map: &HashMap<u32, Vec<u32>>,
    dir: bool,
) -> Option<bool> {
    let page = &update[idx];
    let come_after = left_map.get(page).unwrap_or(default_vec);
    let come_before = right_map.get(page).unwrap_or(default_vec);
    let rem_pages = &update[idx + 1..update.len()];
    let prev_pages = &update[0..idx];

    if dir == false {
        if prev_pages.iter().any(|page| come_after.contains(page)) {
            return Some(false);
        }
    } else {
        if rem_pages.iter().any(|page| come_before.contains(page)) {
            return Some(true);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
