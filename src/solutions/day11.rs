use std::collections::HashMap;
use crate::util::number_extensions::CalcDigits;
use super::*;

pub struct Day11 {
    input: Vec<u64>
}

impl Day11 {
    pub fn new(input: &str) -> Self {
        let input = input.split(" ").filter_map(|str| str.parse().ok()).collect();
        Self { input }
    }

    fn calculate_stone(num: u64) -> Vec<u64> {
        if num == 0 {
            vec![1]
        } else {
            let num_len = num.number_digits() as u32;
            if num_len % 2 == 0 {
                let half = 10_u64.pow(num_len / 2); 
                let (left, right) = (num / half, num % half);
                vec![left, right]
            } else {
                vec![num * 2024]
            }
        }
    }


    fn solve2(nums: Vec<u64>, iterations: usize) -> u64 {
        let mut history: HashMap<(u64, usize), u64> = HashMap::new();
        nums.into_iter().fold(0, |total, num| total + Self::recursive_blinks(num, iterations, &mut history))
    }

    fn recursive_blinks(num: u64, blinks: usize, history: &mut HashMap<(u64, usize), u64>) -> u64 {
        if blinks == 0 { 1 } 
        else if let Some(val) = history.get(&(num, blinks)) { *val } 
        else {
            let total = Self::calculate_stone(num).into_iter().fold(0, |total, num| total + Self::recursive_blinks(num, blinks - 1, history));
            history.insert((num, blinks), total);
            total
        }
    }

}

impl Solution for Day11 {
    fn part1(&self) -> String { Self::solve2(self.input.clone(), 25).to_string() }
    fn part2(&self) -> String { Self::solve2(self.input.clone(), 75).to_string() }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = "125 17";
    #[test] fn test1() {
        assert_eq!(Day11::new(TEST).part1(), 55312.to_string());
    }
    #[test] fn test3() {
        assert_eq!(Day11::solve2(Day11::new(TEST).input, 6), 22);
    }
    #[test] fn test2() {
        assert_eq!(Day11::new(TEST).part2(), 65601038650482u64.to_string());
    }
}

