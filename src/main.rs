use std::{env, time::Instant};
use aoc_2024::solutions::Solution;

#[tokio::main]
async fn main() {
    use aoc_2024::data_handler::args_handler;
    
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 { 
        println!("{}",aoc_2024::error_handling::Error::NotEnoughArgs(args.len()));
        return 
    }

    match args[1].to_lowercase().as_str() {
        "day" => {
            match args_handler::get_input(&args[2..]) {
                Ok(day) => print_solution(day),
                Err(e) => println!("{e}")
            } 
        },
        "create_input_file" => {
            match args_handler::create_input_file(&args[2..]).await {
                Ok(msg) => println!("{msg}"),
                Err(msg) => println!("{msg}")
            }
        },
        "create_rs_file" => {
            match args_handler::create_rust_file(&args[2..]) {
                Ok(msg) => println!("{msg}"),
                Err(msg) => println!("{msg}")
            }
        },
        _ => return println!("Error: Invalid operation inputted")
    }    
}

fn print_solution(day: Box<dyn Solution>) {
    let timer = Instant::now();
    time_solution("1", day.part1().as_str(), timer.elapsed().as_secs_f64());
    let timer = Instant::now();
    time_solution("2", day.part2().as_str(), timer.elapsed().as_secs_f64());
}

fn time_solution(part: &str, solution: &str, time: f64) {
    match solution {
        "" => println!("Part {} not implemented", part),
        _ => println!("The solution to part {} is: {}\nFinished in: {} seconds", part, solution,time),
    }
}