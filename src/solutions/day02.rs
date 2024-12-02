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

    fn solve1(&self, part2: bool) -> u32 {
        let mut accepted = 0;
        for line in self.input.iter() {
            accepted += if Self::is_safe(line, part2) 
            || Self::is_safe(&line[1..].to_vec(), false) { 
                1 
            } 
            else { 
                0 
            };
        }
        accepted 
    }

    fn is_safe(line: &Vec<u8>, part2: bool) -> bool {
        let ascending = line[0] < line[1];

        for (i, (num1, num2)) in line.iter().tuple_windows().enumerate() {
            if num1.abs_diff(*num2) > 3 || ascending && *num1 >= *num2 || !ascending && *num1 <= *num2 {
                return part2 && Self::part2(line, i)
            }
        }
        true
    }
    
    fn part2(line: &Vec<u8>, i: usize) -> bool {
        for i in i..i+2 {
            let mut new_line = line.clone();
            new_line.remove(i);
            if Self::is_safe(&new_line, false) {
                return true
            }
        }
        return false
    }

}

impl Solution for Day2 {
    fn part1(&self) -> String { self.solve1(false).to_string() }
    fn part2(&self) -> String { self.solve1(true).to_string() }
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

