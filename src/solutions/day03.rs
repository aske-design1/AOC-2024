
use regex::Regex;

use super::*;

pub struct Day3 {
    input: String
}

impl Day3 {
    pub fn new(input: &str) -> Self {
        let input = input.to_string();
        Self { input }
    }

    fn solver(&self, re: Regex) -> u64 {
        let mut active = true;
        let mut total: u64 = 0;

        for captures in re.captures_iter(&self.input) {
            match captures.get(0) {
                Some(str) if str.as_str() == "don't()" => active = false,
                Some(str) if str.as_str() == "do()" => active = true,
                _ => ()
            }
            if !active{ 
                continue 
            } 
            if let (Some(num1), Some(num2)) = (captures.get(1), captures.get(2)) {
                let num1 = num1.as_str().parse::<u64>().unwrap_or(0);
                let num2 = num2.as_str().parse::<u64>().unwrap_or(0);
                total += num1 * num2;
            }
        }
        total
    }


}

impl Solution for Day3 {
    fn part1(&self) -> String { 
        let re = Regex::new(r"mul\((\d+),\s*(\d+)\)").unwrap();
        self.solver(re).to_string() 
    }
    fn part2(&self) -> String { 
        let re = Regex::new(r"don't\(\)|do\(\)|mul\((\d+),\s*(\d+)\)").unwrap();
        self.solver(re).to_string() 
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    
    #[test] fn test1() {
        assert_eq!(Day3::new(TEST).part1(), 161.to_string());
    }
    #[test] fn test2() {
        assert_eq!(Day3::new(TEST2).part2(), 48.to_string());
    }
}

