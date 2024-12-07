
use std::collections::HashSet;

use super::*;

pub struct Day6 {
    input: Vec<Vec<u8>>
}

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
        let mut steps = HashSet::new();

        loop {
            steps.insert((x, y));
            if y == 0 || x == 0 || y + 1 == grid.len() || x + 1 == grid.len() {
                break;
            }

            match dirs[dir_idx] {
                North => {
                    match grid[y - 1][x] {
                        b'#' => { 
                            
        
                            dir_idx = (dir_idx + 1) % 4;
                            continue;
                        },
                        _ => y -= 1
                    }
                },
                East => {
                    match grid[y][x + 1] {
                        b'#' => { 
                            
        
                            dir_idx = (dir_idx + 1) % 4;
                            continue;
                        },
                        _ => x += 1
                    }
                },
                South => {
                    match grid[y + 1][x] {
                        b'#' => { 
                            
        
                            dir_idx = (dir_idx + 1) % 4;
                            continue;
                        },
                        _ => y += 1
                    }
                },
                West => {
                    match grid[y][x - 1] {
                        b'#' => { 
                            
        
                            dir_idx = (dir_idx + 1) % 4;
                            continue;
                        },
                        _ => x -= 1
                    }
                } ,
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
        
        let mut obstructions = HashSet::new();

        loop {
            if !obstructions.contains(&(x,y)) && Self::check_cycle(grid,x, y, start_x, start_y) {
                println!("{x} {y}");

                obstructions.insert((x, y));
            }

            if y == 0 || x == 0 || y + 1 == grid.len() || x + 1 == grid.len() {
                break;
            }

            

            match dirs[dir_idx] {
                North => {
                    match grid[y - 1][x] {
                        b'#' => { dir_idx = (dir_idx + 1) % 4; continue },
                        _ => y -= 1
                    }
                },
                East => {
                    match grid[y][x + 1] {
                        b'#' => { dir_idx = (dir_idx + 1) % 4; continue },
                        _ => x += 1
                    }
                },
                South => {
                    match grid[y + 1][x] {
                        b'#' => { 
                            dir_idx = (dir_idx + 1) % 4;
                            continue;
                        },
                        _ => y += 1
                    }
                },
                West => {
                    match grid[y][x - 1] {
                        b'#' => { 
                            
        
                            dir_idx = (dir_idx + 1) % 4;
                            continue;
                        },
                        _ => x -= 1
                    }
                } ,
            }
        }
        obstructions.len() as u32 
    }

    fn check_cycle(grid: &Vec<Vec<u8>>, cur_x: usize, cur_y: usize, end_x: usize, end_y: usize) -> bool {
        if cur_x == end_x && cur_y == end_y { return false }
        
        use Directions::*;
        let dirs = [North, East, South, West];
        let mut dir_idx = 0;      

        let (mut x, mut y) = (cur_x, cur_y);

        let mut steps = 0; 
        while steps < 5403 {
            if x == end_x && y == end_y {
                return true
            }
            
            if y == 0 || x == 0 || y + 1 == grid.len() || x + 1 == grid.len() {
                return false;
            }

            match dirs[dir_idx] {
                North => {
                    match grid[y - 1][x] {
                        b'#' => { dir_idx = (dir_idx + 1) % 4; continue },
                        _ => y -= 1
                    }
                },
                East => {
                    match grid[y][x + 1] {
                        b'#' => { dir_idx = (dir_idx + 1) % 4; continue },
                        _ => x += 1
                    }
                },
                South => {
                    match grid[y + 1][x] {
                        b'#' => { 
                            dir_idx = (dir_idx + 1) % 4;
                            continue;
                        },
                        _ => y += 1
                    }
                },
                West => {
                    match grid[y][x - 1] {
                        b'#' => { 
                            
        
                            dir_idx = (dir_idx + 1) % 4;
                            continue;
                        },
                        _ => x -= 1
                    }
                } ,
            }

            steps+=1;
        }
        false

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
    const TEST: &str = "....#.....
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

