use super::Solution;

#[allow(dead_code)]
pub struct Day1 {
    input: String
}

impl Day1 {
    pub fn new(input: &str) -> Self {
        let input = input.to_string();
        Self { input }
    }
}

impl Solution for Day1 {
    fn part1(&self) -> String { "".to_string() }
    fn part2(&self) -> String { "".to_string() } 
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = "INPUT HERE";
    
    #[test] fn test1() {
        assert_eq!(Day1::new(TEST).part1(), 0.to_string());
    }
    #[test] fn test2() {
        assert_eq!(Day1::new(TEST).part2(), 0.to_string());
    }
}