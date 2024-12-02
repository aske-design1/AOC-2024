use itertools::Itertools;

use super::*;

pub struct Day2 {
    input: Vec<Vec<u8>>
}

impl Day2 {
    pub fn new(input: &str) -> Self {
        let splitter = if input.contains("\r\n") { "\r\n" } else { "\n" };

        let input = input.split(splitter).map(|line| line.split_ascii_whitespace().map(|el| el.parse().unwrap()).collect()).collect();
        Self { input }
    }

    fn solve1(&self) -> u32 {
        let mut accepted = 0;
        for line in self.input.iter() {
            accepted += if Self::check_ascending(line) { 1 } 
            else if Self::check_descending(line) { 1 } 
            else { 0 };
        }

        accepted 
    } 

    fn solve2(&self) -> u32 {
        let mut accepted = 0;
        for line in self.input.iter() {
            accepted += if Self::check_ascending2(line) { 1 } 
            else if Self::check_descending2(line) { 1 } 
            else { 0 };
        }

        accepted 
    } 

    fn check_ascending(line: &Vec<u8>) -> bool {
        for (num1, num2) in line.iter().tuple_windows() {
            if *num1 <= *num2 || num1.abs_diff(*num2) > 3 {
                return false
            }
        }
        true
    }
    
    fn check_descending(line: &Vec<u8>) -> bool {
        for (num1, num2) in line.iter().tuple_windows() {
            if *num1 >= *num2 || num1.abs_diff(*num2) > 3 {
                return false
            }
        }
        true
    }

    fn check_ascending2(line: &Vec<u8>) -> bool {
        for (num1, num2) in line.iter().tuple_windows() {
            if *num1 <= *num2 || num1.abs_diff(*num2) > 3 {
                for i in 0..line.len() {
                    let mut new_line = line.clone();
                    new_line.remove(i);
                    if Self::check_ascending(&new_line) {
                        return true
                    }
                }
                return false 
            }
        }
        true
    }
    
    fn check_descending2(line: &Vec<u8>) -> bool {
        for (num1, num2) in line.iter().tuple_windows() {
            if *num1 >= *num2 || num1.abs_diff(*num2) > 3 {
                for i in 0..line.len() {
                    let mut new_line = line.clone();
                    new_line.remove(i);
                    if Self::check_descending(&new_line) {
                        return true
                    }
                }
                return false 
            }
        }
        true
    }

}

impl Solution for Day2 {
    fn part1(&self) -> String { self.solve1().to_string() }
    fn part2(&self) -> String { self.solve2().to_string() }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    
    #[test] fn test1() {
        assert_eq!(Day2::new(TEST).part1(), 2.to_string());
    }
    #[test] fn test2() {
        assert_eq!(Day2::new(TEST).part2(), 4.to_string());
    }
}

