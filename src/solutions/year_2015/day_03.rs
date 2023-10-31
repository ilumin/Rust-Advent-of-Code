use std::collections::HashMap;

fn move_position((x, y): (isize, isize), direction: &str) -> (isize, isize) {
    match direction {
        "^" => (x, y + 1),
        ">" => (x + 1, y),
        "v" => (x, y - 1),
        "<" => (x - 1, y),
        _ => (x, y),
    }
}

fn delivering(
    direction: &str,
    houses: &HashMap<(isize, isize), isize>,
    predicate: impl FnMut(&(usize, &str)) -> bool,
) -> HashMap<(isize, isize), isize> {
    let mut position = (0, 0);
    let mut houses = houses.clone();

    houses.insert(position, 1);

    direction
        .split("")
        .enumerate()
        .filter(predicate)
        .for_each(|(_, d)| {
            position = move_position(position, d);
            houses.entry(position).and_modify(|e| *e += 1).or_insert(1);
        });

    houses
}

#[allow(dead_code)]
pub fn part_one(input: &str) -> usize {
    let mut houses = HashMap::new();

    houses = delivering(input, &houses, |(_, _)| true);

    houses.len()
}

#[allow(dead_code)]
pub fn part_two(input: &str) -> usize {
    let mut houses = HashMap::new();
    let route_for_santa: fn(&(usize, &str)) -> bool = |(i, _)| i % 2 == 0;
    let route_for_robot: fn(&(usize, &str)) -> bool = |(i, _)| i % 2 != 0;

    houses = delivering(input, &houses, route_for_santa);
    houses = delivering(input, &houses, route_for_robot);

    houses.len()
}

#[cfg(test)]
mod test_part_one {
    use crate::problem::load_raw;

    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(2, part_one(">"))
    }

    #[test]
    fn case_2() {
        assert_eq!(4, part_one("^>v<"))
    }

    #[test]
    fn case_3() {
        assert_eq!(2, part_one("^v^v^v^v^v"))
    }

    #[test]
    fn answer() {
        let input = load_raw(2015, 3);
        println!("ANSWER: 2015/03-1 = {}", part_one(&input))
    }
}

#[cfg(test)]
mod test_part_two {
    use crate::problem::load_raw;

    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(3, part_two("^v"))
    }

    #[test]
    fn case_2() {
        assert_eq!(3, part_two("^>v<"))
    }

    #[test]
    fn case_3() {
        assert_eq!(11, part_two("^v^v^v^v^v"))
    }

    #[test]
    fn answer() {
        let input = load_raw(2015, 3);
        println!("ANSWER: 2015/03-2 = {}", part_two(&input))
    }
}
