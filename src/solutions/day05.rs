
use core::num;
use std::collections::HashMap;

use super::*;

#[allow(dead_code)]
pub struct Day5 {
    order: HashMap<i32, Vec<i32>>,
    numbers: Vec<Vec<i32>> 
}

impl Day5 {    
    pub fn new(input: &str) -> Self { 
        let splitter = if input.contains("\r\n") { "\r\n" } else { "\n" };
        let mut order = HashMap::new();

        let (to_order, numbers) = 
        input.split_once("\n\n").unwrap();

        to_order.split(splitter).for_each(|line| {
            let (first_num, sec_num) = line.split_once("|").unwrap();
            let (first_num, sec_num) = (first_num.parse().unwrap(), sec_num.parse().unwrap());
            order.entry(first_num)
            .and_modify(|vector: &mut Vec<i32>| vector.push(sec_num)).or_insert(vec![sec_num]);
            //Second number's value in hash will be negative to tell that it should come after
            order.entry(sec_num)
            .and_modify(|vector: &mut Vec<i32>| vector.push(-1 * first_num)).or_insert(vec![-1 * first_num]);
        });

        let numbers = numbers.split(splitter).map(|line| line.split(",").map(|num| num.parse().unwrap()).collect()).collect();
        Self { order, numbers }
    }


    fn solve1(&self) -> u32 {
        let mut total = 0;
        for line in self.numbers.iter() {
            if Self::check_line(line, &self.order) {
                let len = line.len();
                //Gonna work as line is uneven
                total += line[len / 2] as u32;
            }
        }
        total
    }

    fn solve2(&self) -> u32 {
        let mut total = 0;
        for line in self.numbers.iter() {
            if !Self::check_line(line, &self.order) {
                //Gonna work as line is uneven
                total += Self::sort_arr(line.clone(), &self.order);
            }
        }
        total
    }

    fn sort_arr(mut numbers: Vec<i32>, order: &HashMap<i32, Vec<i32>>) -> u32 {
        let len = numbers.len();
        let mut i = 0; 
        while i < (len / 2) + 1 {
            let Some(nums_to_cmp) = order.get(&numbers[i]) else { i+=1; continue };
            let mut j = i;
            let mut flag = false;

            while j < len {
                if !nums_to_cmp.contains(&(-1 * numbers[j])) { j+=1; continue; }
                flag = true; 
                let mut k = j;
                while i < k {
                    // 0     4
                    // i     j 
                    // 5 2 4 1 3
                    //Swap numbers
                    (numbers[k - 1], numbers[k]) = (numbers[k], numbers[k - 1]); 
                    k-=1;
                }
                i+=1;
            }
            if flag { i = 0; continue }

            i+=1;
        }


        numbers[len / 2] as u32
    }


    fn check_line(numbers: &Vec<i32>, order: &HashMap<i32, Vec<i32>>) -> bool {
        for (i, num_to_compare) in numbers.iter().enumerate() {
            for (j, other_num) in numbers.iter().enumerate() {
                if j == i { continue }

                if let Some(nums) = order.get(num_to_compare) {
                    let mut idx = None;
                    for (k, num) in nums.iter().enumerate() {
                        if *num == *other_num {
                            idx = Some(k);
                            break;
                        }
                    }
                    if idx.is_none() {
                        continue;
                    }

                    if i < j && nums.get(idx.unwrap()).is_some_and(|num: &i32| *num < 0) 
                    || j < i && nums.get(idx.unwrap()).is_some_and(|num| *num > 0) {
                        //println!("{num_to_compare} {other_num} -> {}", nums[0]);

                        return false
                    }
                }
            }
        }

        true
    }

}

impl Solution for Day5 {
    fn part1(&self) -> String { self.solve1().to_string() }
    fn part2(&self) -> String { self.solve2().to_string() }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = 
"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    
    #[test] fn test1() {
        let day = Day5::new(TEST);

        assert_eq!(day.part1(), 143.to_string());
    }
    #[test] fn test2() {
        assert_eq!(Day5::new(TEST).part2(), 123.to_string());
    }
}

