
use super::*;

pub struct Day11 {
    input: Vec<u64>
}

impl Day11 {
    pub fn new(input: &str) -> Self {
        println!("{input}");
        let input = input.split(" ").filter_map(|str| str.parse().ok()).collect();
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

    fn solve1(&self, iterations: usize) -> u64 {
        //Wrong now
        let mut nums = self.input.clone();
        for _ in 0..iterations {
            let len = nums.len();
            for i in 0..len {
                match nums.get_mut(i) {
                    Some(0) => { nums[0] = 1; }
                    Some(num) if Self::num_digits(*num) % 2 == 0 => {
                        let half = 10_u64.pow(Self::num_digits(*num)); 
                        let (left, right) = (*num / half, *num % half);
                        *num = left;
                        nums.push(right);
                    }
                    Some(num) => { *num *= 2024; }
                    None => (),
                }
            }
        }
        nums.len() as u64
    }
}

impl Solution for Day11 {
    fn part1(&self) -> String { self.solve1(25).to_string() }
    fn part2(&self) -> String { self.solve1(75).to_string() }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = "125 17";
    #[test] fn test1() {
        assert_eq!(Day11::new(TEST).part1(), 55312.to_string());
    }
    #[test] fn test3() {
        assert_eq!(Day11::new(TEST).solve1(6), 22);
    }
    #[test] fn test2() {
        assert_eq!(Day11::new(TEST).part2(), 0.to_string());
    }
}

