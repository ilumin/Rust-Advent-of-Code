use std::fs;

pub trait Solution {
    fn part_one(&self) -> String;
    fn part_two(&self) -> String;
}

pub fn load_raw(year: u32, day: u32) -> String {
    let file = format!("input/{year}/day-{day:02}.txt");
    fs::read_to_string(&file).unwrap_or_else(|_| panic!("Error reading file {}", file))
}
