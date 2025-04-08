
use super::*;

pub struct Day7 {
    input: Vec<Vec<u64>>
}

impl Day7 {
    pub fn new(input: &str) -> Self {
        let splitter = if input.contains("\r\n") { "\r\n" } else { "\n" };
        let input = input.split(splitter).map(|line| {
            let (num1, num2) = line.split_once(": ").unwrap();
            let num1 = num1.parse().unwrap();
            let num2_vec = num2.split_ascii_whitespace().map(|num| num.parse().unwrap()).collect();
           
            [vec![num1], num2_vec].concat()
        }).collect();

        //Put into one vec, where the first num is the results
        Self { input }
    }

    fn num_digits(mut n: u64) -> u32 {
        if n == 0 {
            return 1;
        }
        let mut digits = 0;
        while n > 0 {
            n /= 10;
            digits += 1;
        }
        digits as u32
    }

    fn check_equation(result: u64, accumulation: u64, numbers: &[u64]) -> bool {
        if numbers.is_empty() { return result == accumulation }
        if  result < accumulation { return false }

        Self::check_equation(result, accumulation + numbers[0], &numbers[1..]) ||
        Self::check_equation(result, accumulation * numbers[0], &numbers[1..])
    }

    fn check_equation2(result: u64, accumulation: u64, numbers: &[u64]) -> bool {
        if numbers.is_empty() { return result == accumulation } 
        if  result < accumulation { return false }

        Self::check_equation2(result, accumulation + numbers[0], &numbers[1..]) ||
        Self::check_equation2(result, accumulation * numbers[0], &numbers[1..]) ||
        Self::check_equation2(result, accumulation * 10u64.pow(Self::num_digits(numbers[0])) + numbers[0], &numbers[1..])
    }

    fn solve1(&self) -> u64 { self.input.iter().fold(0, |acc, nums| acc + nums[0] * (Self::check_equation(nums[0], 0,  &nums[1..]) as u64)) }
    fn solve2(&self) -> u64 { self.input.iter().fold(0, |acc, nums| acc + nums[0] * (Self::check_equation2(nums[0], 0,  &nums[1..]) as u64)) }
}

impl Solution for Day7 {
    fn part1(&self) -> String { self.solve1().to_string() }
    fn part2(&self) -> String { self.solve2().to_string() }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    
    #[test] fn test1() {
        assert_eq!(Day7::new(TEST).part1(), 3749.to_string());
    }
    #[test] fn test2() {
        assert_eq!(Day7::new(TEST).part2(), 11387.to_string());
    }
}

