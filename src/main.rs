use std::error::Error;
use std::fs;
use std::io;

mod day01;
mod day02;
mod day03;
mod day07;

fn load_input(day: usize, part: usize) -> io::Result<String> {
    let path = format!("data/day{:02}-{}.txt", day, part);
    fs::read_to_string(path)
}

fn main() -> Result<(), Box<dyn Error>> {
    for (day, part,day_func) in [
        (1usize,1usize,day01::part1 as fn(&str) -> Result<usize,Box<dyn Error>>),
        (1,2,day01::part2 as fn(&str) -> Result<usize,Box<dyn Error>>),
        (2,1,day02::part1 as fn(&str) -> Result<usize,Box<dyn Error>>),
        (2,2,day02::part2 as fn(&str) -> Result<usize,Box<dyn Error>>),
        (3,1,day03::part1 as fn(&str) -> Result<usize,Box<dyn Error>>),
        (3,2,day03::part2 as fn(&str) -> Result<usize,Box<dyn Error>>),
        (7,1,day07::part1 as fn(&str) -> Result<usize,Box<dyn Error>>),
        (7,2,day07::part2 as fn(&str) -> Result<usize,Box<dyn Error>>),
    ] {
        println!("Result for Day {}, Part {}", day, part);
        println!("| Output A: {}", day_func(&load_input(day, 1)?)?);
        println!("| Output B: {}", day_func(&load_input(day, 2)?)?);
    } 
    Ok(())
}