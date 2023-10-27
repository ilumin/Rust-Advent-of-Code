const OPEN_PARENTHESIS: char = '(';
const CLOSE_PARENTHESIS: char = ')';

#[allow(dead_code)]
pub fn part_one(input: &str) -> isize {
    match input.len() == 0 {
        true => 0,
        false => {
            let iter_input = input.chars();
            let open = iter_input
                .clone()
                .filter(|c| c.to_owned() == OPEN_PARENTHESIS)
                .count() as isize;
            let close = iter_input
                .clone()
                .filter(|c| c.to_owned() == CLOSE_PARENTHESIS)
                .count() as isize;

            open - close
        }
    }
}

fn travel(x: &char) -> isize {
    match x.to_owned() {
        OPEN_PARENTHESIS => 1,
        CLOSE_PARENTHESIS => -1,
        _ => 0,
    }
}

#[allow(dead_code)]
pub fn part_two(input: &str) -> usize {
    let mut floor = 0;
    let position = input
        .chars()
        .position(|c| {
            floor += travel(&c);
            floor == -1
        })
        .unwrap();

    position + 1
}

#[cfg(test)]
mod test_part_one {
    use super::*;
    use crate::problem::load_raw;

    #[test]
    fn zero_for_empty() {
        assert_eq!(0, part_one(""))
    }

    #[test]
    fn one_for_open_parenthesis() {
        assert_eq!(1, part_one("("))
    }

    #[test]
    fn negative_one_for_close_parenthesis() {
        assert_eq!(-1, part_one(")"))
    }

    #[test]
    fn validate() {
        assert_eq!(0, part_one("(())"));
        assert_eq!(0, part_one("()()"));
        assert_eq!(3, part_one("((("));
        assert_eq!(3, part_one("(()(()("));
        assert_eq!(3, part_one("))((((("));
        assert_eq!(-1, part_one("())"));
        assert_eq!(-1, part_one("))("));
        assert_eq!(-3, part_one(")))"));
        assert_eq!(-3, part_one(")())())"));
    }

    #[test]
    fn answer() {
        let input = load_raw(2015, 1);
        let answer = part_one(&input);
        println!("ANSWER: 2015/01-1 = {}", answer);
        assert_eq!(answer, 232)
    }
}

#[cfg(test)]
mod test_part_two {
    use super::*;
    use crate::problem::load_raw;

    #[test]
    fn case_1() {
        assert_eq!(1, part_two(")"))
    }

    #[test]
    fn case_2() {
        assert_eq!(5, part_two("()())"))
    }

    #[test]
    fn answer() {
        let input = load_raw(2015, 1);
        let answer = part_two(&input);
        println!("ANSWER: 2015/01-2 = {}", answer);
        assert_eq!(answer, 1783)
    }
}
