const OPEN_PARENTHESIS: &char = &'(';
const CLOSE_PARENTHESIS: &char = &')';

#[allow(dead_code)]
pub fn part_one(input: &str) -> isize {
    match input.len() == 0 {
        true => 0,
        false => {
            let iter_input = input.chars();
            let open = iter_input.clone().filter(|c| c == OPEN_PARENTHESIS).count() as isize;
            let close = iter_input
                .clone()
                .filter(|c| c == CLOSE_PARENTHESIS)
                .count() as isize;

            open - close
        }
    }
}

#[allow(dead_code)]
pub fn part_two() -> String {
    todo!()
}

#[cfg(test)]
mod tests {
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
    fn day_01_part_one() {
        let input = load_raw(2015, 1);
        let answer = part_one(&input);
        println!("2015-day-01-part-1: {}", answer);
        assert_eq!(answer, 232)
    }
}
