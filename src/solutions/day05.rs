
use super::*;

#[allow(dead_code)]
pub struct Day5 {
    input: String
}

impl Day5 {    
    pub fn new(input: &str) -> Self { 
        //let splitter = if input.contains("\r\n") { "\r\n" } else { "\n" };
        let input = input.to_string();
        Self { input }
    }
}

impl Solution for Day5 {
    fn part1(&self) -> String { 0.to_string() }
    fn part2(&self) -> String { 0.to_string() }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = "INPUT HERE";
    
    #[test] fn test1() {
        assert_eq!(Day5::new(TEST).part1(), 0.to_string());
    }
    #[test] fn test2() {
        assert_eq!(Day5::new(TEST).part2(), 0.to_string());
    }
}

