//todo write tests

use std::{env, time::Instant};
use aoc_2024::{
    error_handling::Error,
    data_handler::args_handler,
    solutions::Solution
};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 { 
        return println!("{}", Error::NotEnoughArgs(args.len()))
    }

    match args[1].to_lowercase().as_str() {
        "day" => {
            match args_handler::get_input(&args[2..]).await {
                Ok(day) => print_solution(day),
                Err(e) => println!("{e}")
            } 
        },
        "create_input_file" => {
            match args_handler::create_input_file(&args[2..]).await {
                Ok(msg) if msg.contains("File already exists") => println!("{msg}"),
                Ok(msg) => println!("File Successfully Created\nContent: {}", &msg[..if msg.len() > 100 { 100 } else { msg.len() }]),
                Err(msg) => println!("{msg}")
            }
        },
        "create_rs_file" => {
            match args_handler::create_rust_file(&args[2..]) {
                Ok(msg) => println!("{msg}"),
                Err(msg) => println!("{msg}")
            }
        },
        op => return println!("{}", Error::InvalidOperation(op.to_string()))
    }
}

fn print_solution(day: Box<dyn Solution>) {
    let timer = Instant::now();
    time_solution("1", day.part1().as_str(), timer.elapsed().as_micros());
    let timer = Instant::now();
    time_solution("2", day.part2().as_str(), timer.elapsed().as_micros());
}

fn time_solution(part: &str, solution: &str, time: u128) {
    match solution {
        "" => println!("Part {} not implemented", part),
        _ => println!("The solution to part {} is: {}\nFinished in: {}µs", part, solution,time),
    }
}