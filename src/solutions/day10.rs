
use super::*;

pub struct Day10 {
    input: Vec<String>
}

impl Day10 {
    pub fn new(input: &str) -> Self {
        let splitter = if input.contains("\r\n") { "\r\n" } else { "\n" };
        let input = input.split(splitter).map(String::from).collect();
        Self { input }
    }
}

impl Solution for Day10 {
    fn part1(&self) -> String { 0.to_string() }
    fn part2(&self) -> String { 0.to_string() }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = 
"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    
    #[test] fn test1() {
        assert_eq!(Day10::new(TEST).part1(), 36.to_string());
    }
    #[test] fn test2() {
        assert_eq!(Day10::new(TEST).part2(), 0.to_string());
    }
}

