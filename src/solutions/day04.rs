
use super::*;

pub struct Day4 {
    input: Vec<Vec<char>>
}

impl Day4 {
    pub fn new(input: &str) -> Self {
        let splitter = if input.contains("\r\n") { "\r\n" } else { "\n" };
        let input = input.split(splitter).map(|line| line.chars().collect()).collect();
        Self { input }
    }

    fn solver1(&self) -> u32 {
        let mut total = 0; 
        let grid = &self.input;

        for (i, line) in grid.iter().enumerate() {
            for (j, char) in line.iter().enumerate() {
                if *char == 'X' {
                    total += Self::check_valid((j,i), grid);
                }
            }
        }

        total
    }
    fn solver2(&self) -> u32 {
        let mut total = 0; 
        let grid = &self.input;

        for (i, line) in grid.iter().enumerate() {
            for (j, char) in line.iter().enumerate() {
                if *char == 'A' {
                    total += Self::check_valid2((j,i), grid);
                }
            }
        }

        total
    }

    fn check_valid(cord: (usize, usize), grid: &Vec<Vec<char>>) -> u32 {
        let (x, y) = cord; 
        let valid_forward = 3 + x < grid[y].len();
        let valid_backward = (x as i32) - 3 >= 0;
        let valid_up = (y as i32) - 3 >= 0;
        let valid_down = 3 + y < grid.len();

        let mut valid = 0; 

        if valid_forward && "XMAS" == format!("{}{}{}{}", 
        grid[y][x], grid[y][x + 1], grid[y][x + 2], grid[y][x + 3]).as_str() {
            valid += 1;
        }
        
        if valid_backward && "XMAS" == format!("{}{}{}{}", 
        grid[y][x], grid[y][x - 1], grid[y][x - 2], grid[y][x-3]).as_str() {
            valid += 1;
        }
        
        if valid_up && 
        "XMAS" == format!("{}{}{}{}", 
        grid[y][x], grid[y-1][x], grid[y-2][x], grid[y-3][x]).as_str() {
            valid += 1;
        }
        
        if valid_down && "XMAS" == format!("{}{}{}{}", 
        grid[y][x], grid[y+1][x], grid[y+2][x], grid[y+3][x]).as_str() {
            valid += 1;
        }
        if valid_forward && valid_up && "XMAS" == format!("{}{}{}{}", 
        grid[y][x], grid[y-1][x+1], grid[y-2][x+2], grid[y-3][x+3]).as_str() {
            valid += 1;
        }
        if valid_forward && valid_down && "XMAS" == format!("{}{}{}{}", 
        grid[y][x], grid[y+1][x+1], grid[y+2][x+2], grid[y+3][x+3]).as_str() {
            valid += 1;
        }
        if valid_backward && valid_up && "XMAS" == format!("{}{}{}{}", 
        grid[y][x], grid[y-1][x-1], grid[y-2][x-2], grid[y-3][x-3]).as_str() {
            valid += 1;
        }
        if valid_backward && valid_down && "XMAS" == format!("{}{}{}{}", 
        grid[y][x], grid[y+1][x-1], grid[y+2][x-2], grid[y+3][x-3]).as_str() {
            valid += 1;
        }

        valid
    }

    fn check_valid2(cord: (usize, usize), grid: &Vec<Vec<char>>) -> u32 {
        let (x, y) = cord; 
        let valid_forward = 1 + x < grid[y].len();
        let valid_backward = (x as i32) - 1 >= 0;
        let valid_up = (y as i32) - 1 >= 0;
        let valid_down = 1 + y < grid.len();

        if !valid_forward || !valid_backward || !valid_up || !valid_down {
            return 0
        }


        if  "MAS" == format!("{}{}{}", grid[y-1][x-1], grid[y][x],grid[y+1][x+1] ) 
        && "MAS" == format!("{}{}{}", grid[y-1][x+1], grid[y][x], grid[y+1][x-1] ) {
            return 1   
        }

        if  "MAS" == format!("{}{}{}", grid[y-1][x+1], grid[y][x],grid[y+1][x-1] ) 
        && "MAS" == format!("{}{}{}", grid[y+1][x+1], grid[y][x], grid[y-1][x-1] ) {
            return 1   
        }

        if  "MAS" == format!("{}{}{}", grid[y+1][x-1], grid[y][x],grid[y-1][x+1] ) 
        && "MAS" == format!("{}{}{}", grid[y+1][x+1], grid[y][x], grid[y-1][x-1] ) {
            return 1   
        }

        if  "MAS" == format!("{}{}{}", grid[y-1][x-1], grid[y][x],grid[y+1][x+1] ) 
        && "MAS" == format!("{}{}{}", grid[y+1][x-1], grid[y][x], grid[y-1][x+1] ) {
            return 1   
        }


        return 0
    }



}

impl Solution for Day4 {
    fn part1(&self) -> String { self.solver1().to_string() }
    fn part2(&self) -> String { self.solver2().to_string() }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    
    #[test] fn test1() {
        assert_eq!(Day4::new(TEST).part1(), 18.to_string());
    }
    #[test] fn test2() {
        assert_eq!(Day4::new(TEST).part2(), 9.to_string());
    }
}

