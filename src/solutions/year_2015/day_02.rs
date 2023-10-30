use std::usize;

#[allow(dead_code)]
fn load_input(raw: &str) -> Vec<[usize; 3]> {
    let input: Vec<[usize; 3]> = raw
        .lines()
        .map(|line| {
            let mut split = line.split('x').map(|s| s.parse().unwrap());

            [
                split.next().unwrap(),
                split.next().unwrap(),
                split.next().unwrap(),
            ]
        })
        .collect();

    input
}

fn find_two_smallest_side(sides: [&usize; 3]) -> [&usize; 2] {
    let mut x = sides;
    x.sort();

    [x[0], x[1]]
}

fn find_slack(sides: [&usize; 3]) -> usize {
    let [s1, s2] = find_two_smallest_side(sides);
    s1 * s2
}

// find total square feet
#[allow(dead_code)]
pub fn part_one(input: Vec<[usize; 3]>) -> usize {
    input
        .iter()
        .map(|[l, w, h]| {
            let area = 2 * l * w + 2 * w * h + 2 * h * l;
            let slack = find_slack([l, w, h]);

            area + slack
        })
        .sum()
}

#[cfg(test)]
mod test_part_one {
    use crate::problem::load_raw;

    use super::*;

    #[test]
    fn case_1() {
        let input = load_input("2x3x4");

        assert_eq!(58, part_one(input))
    }

    #[test]
    fn case_2() {
        let input = load_input("1x1x10");

        assert_eq!(43, part_one(input))
    }

    #[test]
    fn answer() {
        let raw = load_raw(2015, 2);
        let input = load_input(&raw);

        println!("ANSWER: 2015/02-1 = {}", part_one(input))
    }
}
