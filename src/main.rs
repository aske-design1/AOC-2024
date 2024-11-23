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
    time_solution("1", day.part1().as_str());
    time_solution("2", day.part2().as_str());
}

fn time_solution(part: &str, solution: &str) {
    let timer = Instant::now();
    match solution {
        "" => println!("Part {} not implemented", part),
        _ => println!("The solution to part {} is: {}", part, solution),
    }
    println!("Time it took: {}", timer.elapsed().as_secs_f64());
}