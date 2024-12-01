use std::collections::HashMap;

use super::Solution;

#[allow(dead_code)]
pub struct Day1 {
    right: Vec<u32>,
    left: Vec<u32>
}

impl Day1 {
    pub fn new(input: &str) -> Self {
        let splitter = if input.contains("\r\n") { "\r\n" } else { "\n" }; 

        let input: (Vec<u32>, Vec<u32>) = input.split(splitter)
        .map(|line| {
            let Some((num1, num2)) = line.split_once("   ") else { panic!("not valid input") } ;

            (num1.parse::<u32>().unwrap(), num2.parse::<u32>().unwrap())
        }).collect();
        Self { right: input.0, left: input.1 }
    }

    fn solve1(&self) -> u32 {
        let (mut column1, mut column2)
        = (self.right.clone(), self.left.clone());
        column1.sort();
        column2.sort();
        column1.iter().zip(column2).map(|(a, b)| a.abs_diff(b)).sum()
    }

    fn solve2(&self) -> u32 {
        let mut column2 = HashMap::with_capacity(1000); 
        self.right.iter()
        .for_each(|el| {
            column2.entry(el).and_modify(|occurrences| *occurrences+=1).or_insert(1);
        });
        self.left.iter().map(|&el| el * *column2.get(&el).unwrap_or(&0)).sum()
    }
}

impl Solution for Day1 {
    fn part1(&self) -> String { self.solve1().to_string() }
    fn part2(&self) -> String { self.solve2().to_string() } 
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = 
"3   4
4   3
2   5
1   3
3   9
3   3";
    
    #[test] fn test1() {
        assert_eq!(Day1::new(TEST).part1(), 11.to_string());
    }
    #[test] fn test2() {
        assert_eq!(Day1::new(TEST).part2(), 31.to_string());
    }
}