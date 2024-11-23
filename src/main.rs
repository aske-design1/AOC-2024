use std::{env, time::Instant};
use aoc_2024::solutions::Solution;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    match aoc_2024::data_handler::args_handler::parse_args(&args).await {
        Ok(day) => print_solution(day),
        Err(e) => println!("Error: {e}")
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