use std::io::Write;

use crate::solutions::Solution;
use crate::solutions::{
    day01, 
    day02,
    day03,
    day04,
    /*day05,
    day06,
    day07,
    day08,
    day09,
    day10,
    day11,
    day12,
    day13,
    day14,
    day15,
    day16,
    day17,
    day18,
    day19,
    day20,
    day21,
    day22,
    day23,
    day24,
    day25*/
};
use crate::error_handling::Error;

type FileResult<T> = Result<T, Error>;


fn create_day_object(day_num: u8, input: String) -> Box<dyn Solution> {
    match day_num {
        1  => Box::new(day01::Day1::new(&input)),
        2  => Box::new(day02::Day2::new(&input)),
        3  => Box::new(day03::Day3::new(&input)),
        4  => Box::new(day04::Day4::new(&input)),/*
        5  => Box::new(day05::Day5::new(&input)),
        6  => Box::new(day06::Day6::new(&input)),
        7  => Box::new(day07::Day7::new(&input)),
        8  => Box::new(day08::Day8::new(&input)),
        9  => Box::new(day09::Day9::new(&input)),
        10 => Box::new(day10::Day10::new(&input)),
        11 => Box::new(day11::Day11::new(&input)),
        12 => Box::new(day12::Day12::new(&input)),
        13 => Box::new(day13::Day13::new(&input)),
        14 => Box::new(day14::Day14::new(&input)),
        15 => Box::new(day15::Day15::new(&input)),
        16 => Box::new(day16::Day16::new(&input)),
        17 => Box::new(day17::Day17::new(&input)),
        18 => Box::new(day18::Day18::new(&input)),
        19 => Box::new(day19::Day19::new(&input)),
        20 => Box::new(day20::Day20::new(&input)),
        21 => Box::new(day21::Day21::new(&input)),
        22 => Box::new(day22::Day22::new(&input)),
        23 => Box::new(day23::Day23::new(&input)),
        24 => Box::new(day24::Day24::new(&input)),
        25 => Box::new(day25::Day25::new(&input)),*/
        _ => unreachable!()
    }
}

pub fn get_input(args: &[String]) -> FileResult<Box<dyn Solution>> {
    let day = &args[0];
    let day_num = day.parse::<u8>()?;

    let path = super::file_handler::create_path("files", "txt", day_num)?;

    if !(0..=25).contains(&day_num) { 
        return Err(Error::InvalidDayNumber(day_num))
    }

    let input = std::fs::read_to_string(path)?;

    Ok(create_day_object(day_num, input))
}

pub async fn create_input_file(args: &[String]) -> FileResult<String> {    
    let day_num = args[0].parse::<u8>()?;
    let path = super::file_handler::create_path("files", "txt", day_num)?;

    if std::fs::read(&path).is_ok() {
        return Ok("File already exists".to_string())
    }

    let content = super::request_handler::get_input(day_num).await?;
    std::fs::File::create(path)?.write_all(content.as_bytes())?;

    Ok("File successfully created".to_string())
}

pub fn create_rust_file(args: &[String]) -> FileResult<String> {
    let day_num = args[0].parse::<u8>()?;
    let path = super::file_handler::create_path("src\\solutions", "rs", day_num)?;

    if std::fs::read(&path).is_ok() {
        return Ok("File already exists".to_string())
    }

    let mut file = std::fs::File::create(path)?;
    let content = super::template::get_template(day_num);
    file.write_all(content.as_bytes())?;

    Ok("File succesfully created".to_string())
}

