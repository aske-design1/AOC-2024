
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

    fn solve1(&self) -> u32 {
        use Directions::*; 

        let dirs = [North, East, South, West];
        let mut dir_idx = 0;      

        let grid = &self.input;
        let (mut y, mut x) = Self::find_pos(grid);
        let mut steps = HashSet::with_capacity(5500);

        loop {
            steps.insert((x, y));
            if y == 0 || x == 0 || y + 1 == grid.len() || x + 1 == grid.len() {
                break;
            }

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

    fn solve2(&self) -> u32 {
        use Directions::*; 

        let dirs = [North, East, South, West];
        let mut dir_idx = 0;      

        let grid = &self.input;
        let (start_y, start_x) = Self::find_pos(grid);
        let (mut y, mut x) = (start_y, start_x);

        let mut counter = 0;

        loop {

            if y == 0 || x == 0 || y + 1 == grid.len() || x + 1 == grid.len() { break }

            if Self::check_valid(grid, &dirs[dir_idx], x, y) && Self::check_cycle(grid,x, y, dir_idx) {
                //println!("{} {}",x,y);
                counter+=1;
            }

            match dirs[dir_idx] {
                North if grid[y - 1][x] != b'#' => y -= 1,
                East  if grid[y][x + 1] != b'#' => x += 1,
                South if grid[y + 1][x] != b'#' => y += 1,
                West  if grid[y][x - 1] != b'#' => x -= 1,
                _ => { dir_idx = (dir_idx + 1) % 4 }
            }

        }
        counter 
    }

    fn check_valid(grid: &Vec<Vec<u8>>, dir: &Directions, x:usize, y:usize) -> bool {
        use Directions::*;
        match dir {
            North if grid[y - 1][x] == b'#' => false,
            East  if grid[y][x + 1] == b'#' => false,
            South if grid[y + 1][x] == b'#' => false,
            West  if grid[y][x - 1] == b'#' => false,
            _ => true
        }
    }

    fn print_grid(grid: &Vec<Vec<u8>>) {
        print!("    ");
        for i in 0..grid.len() {
            print!("{}    ", i);
        }
        println!();

        for (i, line) in grid.iter().enumerate() {
            let line: Vec<char> = line.iter().map(|el| *el as char).collect();
            println!("{} {:?}", i, line);
            
        }


    }

    fn check_cycle(grid: &Vec<Vec<u8>>, start_x: usize, start_y: usize, start_dir: usize) -> bool {
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

        //let mut steps = 0; 
        loop {
            //println!("cur: ({x}, {y}) {dir_idx} --- Start: ({start_x}, {start_y}) {start_dir}");
            if y == 0 || x == 0 || y + 1 == grid.len() || x + 1 == grid.len() {
                return false;
            }

            /*if x == start_x && y == start_y && start_dir == dir_idx {
                //println!("cur: ({x}, {y}) {dir_idx} --- Start: ({start_x}, {start_y}) {start_dir}");
                //println!("Accepted");
                return true
            }*/
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

            
            
            //steps+=1;
        }
        //true

    }


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
    fn part1(&self) -> String { self.solve1().to_string() }
    fn part2(&self) -> String { self.solve2().to_string() }
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
    
    #[test] fn test1() {
        assert_eq!(Day6::new(TEST).part1(), 41.to_string());
    }
    #[test] fn test2() {
        assert_eq!(Day6::new(TEST).part2(), 6.to_string());
    }
}

