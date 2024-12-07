
use core::hash;
use std::collections::HashSet;

use super::*;

pub struct Day6 {
    input: Vec<Vec<u8>>
}

#[derive(PartialEq, Eq)]
enum Directions {
    North,
    East, 
    South,
    West
}

impl Day6 {
    pub fn new(input: &str) -> Self {
        let splitter = if input.contains("\r\n") { "\r\n" } else { "\n" };
        let input = input.split(splitter).map(|line| line.as_bytes().to_vec()).collect();
        Self { input }
    }

    fn solve1(grid: &Vec<Vec<u8>>) -> u32 {
        use Directions::*; 
        let dirs = [North, East, South, West];
        let mut dir_idx = 0;      
    
        let (mut y, mut x) = Self::find_pos(grid);
        let mut steps = HashSet::with_capacity(5500);
        loop {
            steps.insert((x, y));
            if y == 0 || x == 0 || y + 1 == grid.len() || x + 1 == grid.len() { break }
            match dirs[dir_idx] {
                North if grid[y - 1][x] != b'#' => y -= 1,
                East  if grid[y][x + 1] != b'#' => x += 1,
                South if grid[y + 1][x] != b'#' => y += 1,
                West  if grid[y][x - 1] != b'#' => x -= 1,
                _ => dir_idx = (dir_idx + 1) % 4
            }
        }
        steps.len() as u32
    }

    /*fn solve2(&self) -> u32 {
        use Directions::*; 
        let dirs = [North, East, South, West];
        let mut dir_idx = 0;      

        let grid = &self.input;
        let (mut y, mut x) = Self::find_pos(grid);
        let mut no_duplicates = HashSet::new();

        //Self::print_grid(grid);
        loop {
            //println!("{x}, {y}");
            if y == 0 || x == 0 || y + 1 == grid.len() || x + 1 == grid.len() { break }
            if Self::check_valid(grid, &dirs[dir_idx], x, y) {
                let (ob_x, ob_y) = match dirs[dir_idx] {
                    North => (x, y - 1),
                    East  => (x + 1, y),
                    South => (x, y + 1),
                    West  => (x - 1, y),
                };
                if Self::check_cycle3(grid,x, y, ob_x, ob_y, dir_idx) {
                    no_duplicates.insert((x, y)); 
                } 
            }
            match dirs[dir_idx] {
                North if grid[y - 1][x] != b'#' => y -= 1,
                East  if grid[y][x + 1] != b'#' => x += 1,
                South if grid[y + 1][x] != b'#' => y += 1,
                West  if grid[y][x - 1] != b'#' => x -= 1,
                _ => { dir_idx = (dir_idx + 1) % 4 }
            }
        }
        no_duplicates.len() as u32 
    }*/

    fn solve2(mut grid: Vec<Vec<u8>>) -> u32 {        
        let mut valid = 0u32;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == b'#' || grid[i][j] == b'^' { continue } 
                grid[i][j] = b'#'; 
                if Self::check_if_loop(&grid) {
                    valid += 1;
                }
                grid[i][j] = b'.';
            }
        }
        valid
    }

    fn check_if_loop(grid: &Vec<Vec<u8>>) -> bool {
        use Directions::*; 
        let dirs = [North, East, South, West];
        let mut dir_idx = 0;      
        let (mut y, mut x) = Self::find_pos(grid);
        //let mut steps = HashSet::with_capacity(5500);
        
        let mut steps = 0;
        while steps < 10000 {
            if y == 0 || x == 0 || y + 1 == grid.len() || x + 1 == grid.len() { return false }
            match dirs[dir_idx] {
                North if grid[y - 1][x] != b'#' => y -= 1,
                East  if grid[y][x + 1] != b'#' => x += 1,
                South if grid[y + 1][x] != b'#' => y += 1,
                West  if grid[y][x - 1] != b'#' => x -= 1,
                _ => dir_idx = (dir_idx + 1) % 4
            }
            steps+=1;
        }
        true
    }

    /*fn check_valid(grid: &Vec<Vec<u8>>, dir: &Directions, x:usize, y:usize) -> bool {
        use Directions::*;
        match dir {
            North if grid[y - 1][x] == b'#' => false,
            East  if grid[y][x + 1] == b'#' => false,
            South if grid[y + 1][x] == b'#' => false,
            West  if grid[y][x - 1] == b'#' => false,
            _ => true
        }
    }*/

    /*fn print_grid(grid: &Vec<Vec<u8>>) {
        print!("    ");
        for i in 0..grid.len() {
            print!("{}    ", i);
        }
        println!();

        for (i, line) in grid.iter().enumerate() {
            let line: Vec<char> = line.iter().map(|el| *el as char).collect();
            println!("{} {:?}", i, line);
            
        }
    }*/

    /*fn check_cycle(grid: &Vec<Vec<u8>>, start_x: usize, start_y: usize, start_dir: usize) -> bool {
        use Directions::*;
        let dirs = [North, East, South, West];
        let (mut x, mut y) = (start_x, start_y);

        //println!("Start: ({start_x}, {start_y}) {start_dir}");
        let mut grid = grid.clone();
        match dirs[start_dir] {
            North => grid[y - 1][x] = b'#',
            East  => grid[y][x + 1] = b'#',
            South => grid[y + 1][x] = b'#',
            West  => grid[y][x - 1] = b'#',
        }

        let mut dir_idx = (start_dir + 1) % 4;
        //let mut seen = HashSet::new();

        let mut steps = 0; 
        while steps < 6000 {
            //println!("cur: ({x}, {y}) {dir_idx} --- Start: ({start_x}, {start_y}) {start_dir}");
            if y == 0 || x == 0 || y + 1 == grid.len() || x + 1 == grid.len() {
                return false;
            }

            
            /*if !seen.insert((x, y, dir_idx)) {
                return true
            }*/


            match dirs[dir_idx] {
                North if grid[y - 1][x] != b'#' => y -= 1,
                East  if grid[y][x + 1] != b'#' => x += 1,
                South if grid[y + 1][x] != b'#' => y += 1,
                West  if grid[y][x - 1] != b'#' => x -= 1,
                _ => dir_idx = (dir_idx + 1) % 4
            }

            if x == start_x && y == start_y && (start_dir == dir_idx || (start_dir + 1) % 4 == dir_idx ) {
                //println!("cur: ({x}, {y}) {dir_idx} --- Start: ({start_x}, {start_y}) {start_dir}");
                //println!("Accepted");
                return true
            }
            
            
            steps+=1;
        }
        false

    }

    fn check_cycle2(grid: &Vec<Vec<u8>>, start_x: usize, start_y: usize, start_dir: usize) -> bool {
        use Directions::*;
        let dirs = [North, East, South, West];
        let (mut x, mut y) = (start_x, start_y);

        //println!("Start: ({start_x}, {start_y}) {start_dir}");
        let mut grid = grid.clone();
        match dirs[start_dir] {
            North => grid[y - 1][x] = b'#',
            East  => grid[y][x + 1] = b'#',
            South => grid[y + 1][x] = b'#',
            West  => grid[y][x - 1] = b'#',
        }

        let mut dir_idx = (start_dir + 1) % 4;
        let mut seen = HashSet::new();

        loop {
            //println!("cur: ({x}, {y}) {dir_idx} --- Start: ({start_x}, {start_y}) {start_dir}");
            if y == 0 || x == 0 || y + 1 == grid.len() || x + 1 == grid.len() {
                return false;
            }
            if !seen.insert((x, y, dir_idx)) {
                return true
            }
            match dirs[dir_idx] {
                North if grid[y - 1][x] != b'#' => y -= 1,
                East  if grid[y][x + 1] != b'#' => x += 1,
                South if grid[y + 1][x] != b'#' => y += 1,
                West  if grid[y][x - 1] != b'#' => x -= 1,
                _ => dir_idx = (dir_idx + 1) % 4
            }
        }
    }*/

    /*fn check_cycle3(grid: &Vec<Vec<u8>>, start_x: usize, start_y: usize, obstruction_x: usize, obstruction_y: usize, start_dir: usize) -> bool {
        use Directions::*;
        let dirs = [North, East, South, West];
        let mut dir_idx = (start_dir + 1) % 4;
        
        //Fuck it all -> fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck  fuck  fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck
        let (mut x, mut y) = (start_x, start_y);

        //First 1 = x, 2 = y, 3 = start_dir
        let mut visited: HashSet<(usize, usize, usize)> = HashSet::with_capacity(3000);
        visited.insert((x, y, start_dir));
        visited.insert((x, y, dir_idx));

        loop {
            if y == 0 || x == 0 || y + 1 == grid.len() || x + 1 == grid.len() {
                return false;
            }
            match dirs[dir_idx] {
                North if grid[y - 1][x] != b'#' && y - 1 != obstruction_y && x != obstruction_x => y -= 1,
                East  if grid[y][x + 1] != b'#' && y != obstruction_y && x + 1 != obstruction_x => x += 1,
                South if grid[y + 1][x] != b'#' && y + 1 != obstruction_y && x != obstruction_x => y += 1,
                West  if grid[y][x - 1] != b'#' && y != obstruction_y && x - 1 != obstruction_x => x -= 1,
                _ => dir_idx = (dir_idx + 1) % 4
            }

            if !visited.insert((x, y, dir_idx)) { return true }

        }
    }*/




    fn find_pos(grid: &Vec<Vec<u8>>) -> (usize, usize) {
        for (i, line) in grid.iter().enumerate() {
            for (j, byte) in line.iter().enumerate() {
                if *byte == b'^' { return (i, j) }
            }
        }
        return (0,0)
    }
}

impl Solution for Day6 {
    fn part1(&self) -> String { Self::solve1(&self.input).to_string() }
    fn part2(&self) -> String { Self::solve2(self.input.clone()).to_string() }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = 
"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    const TEST2: &str = 
".#......
...#.#..
#......#
......#.
.#......
..^...#.
#...#...
..#..#..";
    
    #[test] fn test1() {
        assert_eq!(Day6::new(TEST).part1(), 41.to_string());
    }
    #[test] fn test2() {
        assert_eq!(Day6::new(TEST).part2(), 6.to_string());
    }

    #[test] fn test3() {
        assert_eq!(Day6::new(TEST2).part2(), 2.to_string());
    }
}

