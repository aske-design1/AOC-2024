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

    /*fn solver(&self, should_ignore: bool) -> u32 {
        let mut accepted = 0;
        for line in self.input.iter() {
            if Self::check_safety(line, line[0] < line[1], should_ignore) ||
            should_ignore && Self::check_safety(&line[1..], line[1] < line[2], false) {
                accepted += 1;
            }
        }
        accepted 
    }*/ 

    fn solver(&self, skip_active: bool) -> u32 {
        let mut accepted = 0;
        for line in self.input.iter() {
            if Self::new_func(line, line[0] < line[1], skip_active) {
                accepted += 1;
            }
        }
        accepted 
    } 

    fn check_safety(line: &[u8], ascend: bool, should_ignore: bool) -> bool {
        if line.len() <= 1 { return true }
        let (num1, num2) = (line[0], line[1]);

        let ascension = ascend && num1 < num2 && num1.abs_diff(num2) <= 3 && 
        Self::check_safety(&line[1..], ascend, should_ignore);
 
        let descension = !ascend && num1 > num2 && num1.abs_diff(num2) <= 3 
        && Self::check_safety(&line[1..], ascend, should_ignore);


        let skip_level = should_ignore && Self::check_safety(
            &line[..1].iter().copied().chain(line[2..].iter().copied()).collect::<Vec<u8>>(), 
            ascend, 
            false
        );

        ascension || descension || skip_level
    }

    fn check_ascending(line: &Vec<u8>) -> bool {
        for (num1, num2) in line.iter().tuple_windows() {
            if *num1 <= *num2 || num1.abs_diff(*num2) > 3 {
                return false
            }
        }
        true
    }

    fn new_func(line: &Vec<u8>, ascend: bool, mut skip_active: bool) -> bool {
        let mut line: Vec<u8> = line.iter().copied().collect();
        let mut i = 1;
        while i < line.len() {
            let mut flag = true;
            let (num1, num2) = (line[i-1], line[i]);

            if num1.abs_diff(num2) > 3 || ascend && num1 >= num2 || !ascend && num1 <= num2 {
                flag = false 
            }

            if flag {
                i+=1;
            } else if !flag && skip_active {
                skip_active = false;
                line.remove(i);
                continue;
            } else {
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
    fn part1(&self) -> String { self.solver(false).to_string() }
    fn part2(&self) -> String { self.solver(true).to_string()  }
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

