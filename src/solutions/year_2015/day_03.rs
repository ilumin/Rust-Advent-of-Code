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

#[allow(dead_code)]
pub fn part_one(input: &str) -> usize {
    let mut position = (0, 0);
    let mut houses = HashMap::new();
    houses.insert(position, 1);

    input.split("").into_iter().for_each(|direction| {
        position = move_position(position, &direction);
        houses.entry(position).and_modify(|e| *e += 1).or_insert(1);
    });

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
        println!("ANSWER: 2015/02-2 = {}", part_one(&input))
    }
}