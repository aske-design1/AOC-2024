
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

    fn solve1(&self, iterations: usize) -> u64 {
        let mut nums = self.input.clone();
        for i in 0..iterations {
            //let mut insert = 0;
            let len = nums.len();
            for i in 0..len {
                match nums.get(i ).unwrap() {
                    0 => { nums.get_mut(i ).map(|val| *val = 1); },
                    num if ((*num as f64).log(10.0).floor() as u32 & 1) == 0 => {
                        //let num_str = num.to_string();
                        let len = (*num as f64).log(10.0).floor() as u64;
                        let (left, right) = ((*num) / len, *num % len);


                        //let (left, right) = num_str.split_at(num_str.len() / 2);
                        //let (left, right) = (left.trim_start_matches("0"), right.trim_start_matches("0"));
                        //nums[i] = left.parse().unwrap_or(0);
                        nums[i] = left; 
                        //insert+=1;
                        //println!("{left}, {right}");
                        //nums.push(right.parse().unwrap_or(0));
                        nums.push(right);
                    }
                    _ => { nums.get_mut(i).map(|val| *val *= 2024); }
                }
            }
            //println!("{}",nums.len());
            println!("{i} ");
        }
        println!();
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
    #[test] fn test2() {
        assert_eq!(Day11::new(TEST).part2(), 0.to_string());
    }
}

