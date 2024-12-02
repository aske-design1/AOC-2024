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

    fn solver(&self, skip_active: bool) -> u32 {
        let mut accepted = 0;
        for line in self.input.iter() {
            if Self::safety_check(line, 0, line[0] < line[1], skip_active) ||
            skip_active && Self::safety_check(&line[1..], 0, line[1] < line[2], false) {
                accepted += 1;
            }
        }
        accepted 
    } 

   
    fn safety_check(line: &[u8], mut i: usize, ascend: bool, skip_active: bool) -> bool {
        while i < line.len() - 1  {
            let mut flag = true;
            let (num1, num2) = (line[i], line[i + 1]);

            if num1.abs_diff(num2) > 3 || ascend && num1 >= num2 || !ascend && num1 <= num2 {
                flag = false 
            }

            if !flag {

                if !skip_active {
                    return false
                }

                let mut vec1 = line.to_vec();
                vec1.remove(i);
                let mut vec2 = line.to_vec();
                vec2.remove(i+1);

                return i != 0 && (i + 1 == line.len() ||
                Self::safety_check(&vec1, i-1, ascend, false) ||
                Self::safety_check(&vec2, i, ascend, false))
            }

            i+=1;
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

