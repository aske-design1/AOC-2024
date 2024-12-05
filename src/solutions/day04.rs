
use super::*;

pub struct Day4 {
    input: String
}

impl Day4 {
    pub fn new(input: &str) -> Self {
        //let splitter = if input.contains("\r\n") { "\r\n" } else { "\n" };
        //let input = input.split(splitter).map(|line| line.as_bytes().to_vec()).collect();
        Self { input: input.to_string() }
    }

    fn solver1(&self) -> u32 {
        let splitter = if self.input.contains("\r\n") { "\r\n" } else { "\n" };
        let grid: Vec<&[u8]> = self.input.split(splitter).map(|line| line.as_bytes()).collect();

        let mut total = 0; 
        for (i, line) in grid.iter().enumerate() {
            for (j, char) in line.iter().enumerate() {
                if *char == b'X' {
                    total += Self::check_valid1((j,i), &grid);
                }
            }
        }
        total
    }

    fn check_valid1(cord: (usize, usize), grid: &Vec<&[u8]>) -> u32 {
        let (x, y) = cord;
        let word = [b'X', b'M', b'A', b'S'];

        //u8 that contains bits for valid dirs: 
        let directions: u8 =  
        //1st bit = forward
        (1 << 0) * ((3 + x < grid[y].len()) as u8) +
        //2nd bit = backward
        (1 << 1) * ((x >= 3) as u8) +
        //3rd bit = up 
        (1 << 2) * ((y >= 3) as u8) +
        //4th bit = down
        (1 << 3) * ((3 + y < grid.len() ) as u8);

        //Forward
        1*((directions & 0b0001 != 0
        && word.iter().enumerate().all(|(i, &ch)| grid[y][x + i] == ch)) as u32)+
        //Backward
        1*((directions & 0b0010 != 0
        && word.iter().enumerate().all(|(i, &ch)| grid[y][x - i] == ch)) as u32)+
        //Up
        1*((directions & 0b0100 != 0
        && word.iter().enumerate().all(|(i, &ch)| grid[y - i][x] == ch)) as u32)+
        //Down
        1*((directions & 0b1000 != 0
        && word.iter().enumerate().all(|(i, &ch)| grid[y + i][x] == ch)) as u32)+
        //Up-forward
        1*((directions & 0b0101 == 0b0101 
        && word.iter().enumerate().all(|(i, &ch)| grid[y - i][x + i] == ch)) as u32)+
        //down-forward
        1*((directions & 0b1001 == 0b1001  
        && word.iter().enumerate().all(|(i, &ch)| grid[y + i][x + i] == ch)) as u32)+
        //up-backward
        1*((directions & 0b0110 == 0b0110 
        && word.iter().enumerate().all(|(i, &ch)| grid[y - i][x - i] == ch)) as u32)+
        //down-backward
        1*((directions & 0b1010 == 0b1010 
        && word.iter().enumerate().all(|(i, &ch)| grid[y + i][x - i] == ch)) as u32)
    }
    

    fn solver2(&self) -> u32 {
        let splitter = if self.input.contains("\r\n") { "\r\n" } else { "\n" };
        //let grid: &[u8] = self.input.as_bytes();
        let grid: Vec<&[u8]> = self.input.split(splitter).map(|line| line.as_bytes()).collect();
        let mut total = 0; 
        //let len = grid.iter().find(|&ch| *ch == b'\n');

        for (i, line) in grid.iter().enumerate().skip(1).take(grid.len() - 2) {
            for (j, char) in line.iter().enumerate().skip(1).take(line.len() - 2) {
                if *char == b'A' { 
                    total += Self::check_valid2((j,i), &grid) 
                }
            }
        }
        total
    }
    fn check_valid2(cord: (usize, usize), grid: &Vec<&[u8]>) -> u32 {
        let (x, y) = cord; 
        //Four cases:
        //M   M     S   M    S   S    M   S
        //  A         A        A        A
        //S   S     S   M    M   M    M   S
        if b'M' == grid[y-1][x-1] && b'M' == grid[y-1][x+1]
        && b'S' == grid[y+1][x-1] && b'S' == grid[y+1][x+1]
        || b'M' == grid[y-1][x+1] && b'M' == grid[y+1][x+1] 
        && b'S' == grid[y-1][x-1] && b'S' == grid[y+1][x-1]
        || b'M' == grid[y+1][x-1] && b'M' == grid[y+1][x+1]
        && b'S' == grid[y-1][x-1] && b'S' == grid[y-1][x+1] 
        || b'M' == grid[y-1][x-1] && b'M' == grid[y+1][x-1]
        && b'S' == grid[y-1][x+1] && b'S' == grid[y+1][x+1]
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

