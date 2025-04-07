
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

use super::*;

pub struct Day8 {
    input: Vec<String>
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Point { x: value.0 as i32, y: value.1 as i32 }
    }
}

impl From<(i32,i32)> for Point {
    fn from(value: (i32,i32)) -> Self {
        let (x, y) = value;
        Point { x, y }
    }
}


impl Point {
    fn check_bound(self, point: &Point) -> Option<Self> {
        if self.x < 0 || self.y < 0 ||  self.x >= point.x || self.y >= point.x { None }
        else { Some(self) }
    }

    fn calc_freq(&self, tower: &Point, grid_size: &Self) -> (Option<Self>, Option<Self>) {
        let (x_dif, y_dif) = (self.x - tower.x, self.y - tower.y);
        (  
            Point::from((self.x + x_dif, self.y + y_dif)).check_bound(grid_size), 
            Point::from((tower.x - x_dif, tower.y - y_dif)).check_bound(grid_size)
        )
    }

    fn calc_repeat_freq(&self, tower: &Point, grid_size: &Self, positions: &mut HashSet<Point>) {
        let mut collect_frequencies = |mut current: Option<Point>, dx: i32, dy: i32| {
            while let Some(point) = current {
                current = Point::from((point.x + dx, point.y + dy)).check_bound(grid_size);
                positions.insert(point);
            }
        };
        let (x_dif, y_dif) = (self.x - tower.x, self.y - tower.y);

        collect_frequencies(Some(self.clone()), x_dif, y_dif);
        collect_frequencies(Some(tower.clone()), -x_dif, -y_dif);
    }
}


impl Day8 {
    pub fn new(input: &str) -> Self {
        let splitter = if input.contains("\r\n") { "\r\n" } else { "\n" };
        let input = input.split(splitter).map(|line| line.to_string()).collect();
        Self { input }
    }

    fn solve1(&self) -> u32 {
        let grid_size = Point::from((self.input[0].len(), self.input.len()));
        Self::find_points(&self.input).values()
        .fold(HashSet::with_capacity(500), |mut acc: HashSet<Point>, points | {
            points.iter().combinations(2).for_each(|pair| {                
                let (fir_sig, sec_sig) = pair[0].calc_freq(pair[1], &grid_size);
                fir_sig.map(|val| acc.insert(val)); 
                sec_sig.map(|val| acc.insert(val));                
            });
            acc
        }).len() as u32 
    }

    fn find_points(map: &Vec<String>) -> HashMap<char, Vec<Point>> {
        map.iter().enumerate()
        .flat_map(|(y, line)|
            line.chars().enumerate()
                .filter_map(move |(x, ch)| 
                    if ch != '.' { Some((ch, Point::from((x, y)))) } 
                    else { None }
                )
        ).fold(HashMap::with_capacity(500), |mut acc, (ch, point)| {
            acc.entry(ch).or_insert_with(Vec::new).push(point);
            acc
        })
    }

    fn solve2(&self) -> u32 {
        let grid_size = Point::from((self.input[0].len(), self.input.len()));
        Self::find_points(&self.input).values()
        .fold(HashSet::with_capacity(1000), |mut acc: HashSet<Point>, points | {
            points.iter().combinations(2).for_each(|pair| {
                let (first, sec) = (pair[0], pair[1]);
                first.calc_repeat_freq(sec, &grid_size, &mut acc);
            });
            acc
        }).len() as u32
    }

}

impl Solution for Day8 {
    fn part1(&self) -> String { self.solve1().to_string() }
    fn part2(&self) -> String { self.solve2().to_string() }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = 
"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    
    #[test] fn test1() {
        assert_eq!(Day8::new(TEST).part1(), 14.to_string());
    }
    #[test] fn test2() {
        assert_eq!(Day8::new(TEST).part2(), 34.to_string());
    }
}

