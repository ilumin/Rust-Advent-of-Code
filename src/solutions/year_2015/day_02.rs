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

fn find_area([l, w, h]: &[usize; 3]) -> usize {
    2 * l * w + 2 * w * h + 2 * h * l
}

fn find_two_smallest_side(dimension: &[usize; 3]) -> (usize, usize) {
    let mut dim = *dimension;
    dim.sort();

    (dim[0], dim[1])
}

fn find_slack(dimension: &[usize; 3]) -> usize {
    let (s1, s2) = find_two_smallest_side(dimension);
    s1 * s2
}

fn find_ribbon(dimension: &[usize; 3]) -> usize {
    let (s1, s2) = find_two_smallest_side(dimension);
    2 * (s1 + s2)
}

fn find_bow(dimension: &[usize; 3]) -> usize {
    dimension.iter().product()
}

fn find_wrapping_paper_size(dimension: &[usize; 3]) -> usize {
    find_area(dimension) + find_slack(dimension)
}

fn find_ribbon_size(dimension: &[usize; 3]) -> usize {
    find_ribbon(dimension) + find_bow(dimension)
}

// find total square feet
#[allow(dead_code)]
pub fn part_one(input: Vec<[usize; 3]>) -> usize {
    input.iter().map(find_wrapping_paper_size).sum()
}

#[allow(dead_code)]
pub fn part_two(input: Vec<[usize; 3]>) -> usize {
    input.iter().map(find_ribbon_size).sum()
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

#[cfg(test)]
mod test_part_two {
    use crate::problem::load_raw;

    use super::*;

    #[test]
    fn case_1() {
        let input = load_input("2x3x4");

        assert_eq!(34, part_two(input))
    }

    #[test]
    fn case_2() {
        let input = load_input("1x1x10");

        assert_eq!(14, part_two(input))
    }

    #[test]
    fn answer() {
        let raw = load_raw(2015, 2);
        let input = load_input(&raw);

        println!("ANSWER: 2015/02-2 = {}", part_two(input))
    }
}
