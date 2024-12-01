use std::collections::HashMap;

use super::Solution;

#[allow(dead_code)]
pub struct Day1 {
    input: Vec<(u32, u32)>
}

impl Day1 {
    pub fn new(input: &str) -> Self {
        let splitter = if input.contains("\r\n") { "\r\n" } else { "\n" }; 

        let input = input.split(splitter)
        .map(|line| {
            let Some((num1, num2)) = line.split_once("   ") else { panic!("not valid input") } ;

            (num1.parse().unwrap(), num2.parse().unwrap())
        }).collect();
        Self { input }
    }

    fn solve1(&self) -> u32 {
        let mut column1 = Vec::new(); 
        let mut column2 = Vec::new(); 
        for line in self.input.iter() {
            column1.push(line.0);
            column2.push(line.1);
        }

        column1.sort();
        column2.sort();

        let mut total_dist = 0; 
        for i in 0..self.input.len() {
            let (num1, num2) = (column1[i], column2[i]);

            total_dist += if num1 > num2 {
                num1 - num2
            } else {
                num2 - num1
            };
            
        }
        return total_dist;
    }

    fn solve2(&self) -> u32 {
        let mut column1 = Vec::new(); 
        let mut column2 = HashMap::new(); 
        for line in self.input.iter() {
            column1.push(line.0);
            column2.entry(line.1).and_modify(|occurrences| *occurrences+=1).or_insert(1);   
        }

        let mut total_dist = 0; 
        for num in column1 {
            total_dist += if let Some(occurences) = column2.get(&num)  {
                num * occurences
            } else { 0 }

        }


        return total_dist;
    }

}

impl Solution for Day1 {
    fn part1(&self) -> String {         
        self.solve1().to_string() 
    }
    fn part2(&self) -> String { self.solve2().to_string() } 
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = "3   4
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