
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
                    total += Self::check_valid1((j,i), grid);
                }
            }
        }
        total
    }
    
    fn check_valid1(cord: (usize, usize), grid: &Vec<Vec<char>>) -> u32 {
        let (x, y) = cord;
        let word = ['X', 'M', 'A', 'S'];
        let mut valid = 0;

        //u8 that contains bits for valid dirs: 
        let mut validizer: u8 = 0; 
        //1st bit = forward
        if 3 + x < grid[y].len() { validizer |= 1 << 0 }
        //2nd bit = backward
        if x >= 3 { validizer |= 1 << 1 }
        //3rd bit = up 
        if y >= 3 { validizer |= 1 << 2 }
        //4th bit = down 
        if 3 + y < grid.len() { validizer |= 1 << 3 }
        
        //Forward
        if validizer & 0b0001 != 0
        && word.iter().enumerate().all(|(i, &ch)| grid[y][x + i] == ch) 
        { valid += 1; }
        //Backward
        if validizer & 0b0010 != 0
        && word.iter().enumerate().all(|(i, &ch)| grid[y][x - i] == ch) 
        { valid += 1 }
        //Up
        if validizer & 0b0100 != 0
        && word.iter().enumerate().all(|(i, &ch)| grid[y - i][x] == ch) 
        { valid += 1 }
        //Down
        if validizer & 0b1000 != 0
        && word.iter().enumerate().all(|(i, &ch)| grid[y + i][x] == ch) 
        { valid += 1 }
        //Up-forward
        if validizer & 0b0101 == 0b0101 
        && word.iter().enumerate().all(|(i, &ch)| grid[y - i][x + i] == ch) 
        { valid += 1 }
        //down-forward
        if validizer & 0b1001 == 0b1001  
        && word.iter().enumerate().all(|(i, &ch)| grid[y + i][x + i] == ch) 
        { valid += 1 }
        //up-backward
        if validizer & 0b0110 == 0b0110 
        && word.iter().enumerate().all(|(i, &ch)| grid[y - i][x - i] == ch) 
        { valid += 1 }
        //down-backward
        if validizer & 0b1010 == 0b1010 
        && word.iter().enumerate().all(|(i, &ch)| grid[y + i][x - i] == ch) 
        { valid += 1 }
        valid
    }
    

    fn solver2(&self) -> u32 {
        let mut total = 0; 
        let grid = &self.input;
        for (i, line) in grid.iter().enumerate().skip(1).take(grid.len() - 2) {
            for (j, char) in line.iter().enumerate().skip(1).take(line.len() - 2) {
                if *char == 'A' { 
                    total += Self::check_valid2((j,i), grid) 
                }
            }
        }
        total
    }
    fn check_valid2(cord: (usize, usize), grid: &Vec<Vec<char>>) -> u32 {
        let (x, y) = cord; 
        //Four cases:
        //M   M     S   M    S   S    M   S
        //  A         A        A        A
        //S   S     S   M    M   M    M   S
        if 'M' == grid[y-1][x-1] && 'M' == grid[y-1][x+1]
        && 'S' == grid[y+1][x-1] && 'S' == grid[y+1][x+1]
        || 'M' == grid[y-1][x+1] && 'M' == grid[y+1][x+1] 
        && 'S' == grid[y-1][x-1] && 'S' == grid[y+1][x-1]
        || 'M' == grid[y+1][x-1] && 'M' == grid[y+1][x+1]
        && 'S' == grid[y-1][x-1] && 'S' == grid[y-1][x+1] 
        || 'M' == grid[y-1][x-1] && 'M' == grid[y+1][x-1]
        && 'S' == grid[y-1][x+1] && 'S' == grid[y+1][x+1]
        { 1 } 
        else { 0 }
    }



}

impl Solution for Day4 {
    fn part1(&self) -> String { self.solver1().to_string() }
    fn part2(&self) -> String { self.solver2().to_string() }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = 
"MMMSXXMASM
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

