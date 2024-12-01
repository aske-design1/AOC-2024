
use super::*;

pub struct Day2 {
    input: String
}

impl Day2 {
    pub fn new(input: &str) -> Self {
        let input = input.to_string();
        Self { input }
    }
}

impl Solution for Day2 {
    fn part1(&self) -> String { 0.to_string() }
    fn part2(&self) -> String { 0.to_string() }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = "INPUT HERE";
    
    #[test] fn test1() {
        assert_eq!(Day2::new(TEST).part1(), 0.to_string());
    }
    #[test] fn test2() {
        assert_eq!(Day2::new(TEST).part2(), 0.to_string());
    }
}

