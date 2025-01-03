use std::error::Error;
use regex::Regex;

struct Mul(usize,usize);
enum Command {
  Multiply(Mul),
  Dont,
  Do
}

const MULTIPLY_RE: &str = r"mul\((\d+),(\d+)\)";
const ALL_COMMANDS_RE: &str = r"(mul\(\d+,\d+\)|don[']t\(\)|do\(\))";

fn parse_input(input: String) -> Result<Vec<Mul>,Box<dyn Error>> {
  let re = Regex::new(MULTIPLY_RE).unwrap();
  let mut result: Vec<Mul> = Vec::new();
  for (_, [a,b]) in re.captures_iter(input.as_str()).map(|c| c.extract()) {
    let a2: usize  = a.parse()?;
    let b2: usize = b.parse()?;
    result.push(Mul(a2,b2));
  }
  Ok(result)
}

pub fn part1(input: String) -> Result<usize,Box<dyn Error>> {
  let result = 
    parse_input(input)?
      .iter()
      .map(|Mul(a,b)| a*b)
      .sum::<usize>();
  Ok(result)
}

fn parse_mul(input: &str) -> Result<Mul, Box<dyn Error>> {
  let length = input.chars().count();
  let result = 
    input.chars().skip(4)
      .take(length - 5)
      .collect::<String>();
  let mut split_string = result.as_str()
      .split(',');
  let a = split_string.next().ok_or("Nothing found")?;
  let b = split_string.next().ok_or("Only one value found")?;
  let a2: usize = a.parse()?;
  let b2: usize = b.parse()?;
  Ok(Mul(a2,b2))
}

fn parse_part2(input: String) -> Result<Vec<Command>,Box<dyn Error>> {
  let re = Regex::new(ALL_COMMANDS_RE).unwrap();
  let mut result: Vec<Command> = Vec::new();
  for (_, [command]) in re.captures_iter(input.as_str()).map(|c| c.extract()) {
    match command {
      "don't()" => 
        result.push(Command::Dont),
      "do()" => 
        result.push(Command::Do),
      _ if command.starts_with("mul(") => 
        result.push(Command::Multiply(parse_mul(command)?)),
      _ => continue
    }
  }
  Ok(result)
}

pub fn part2(input: String) -> Result<usize,Box<dyn Error>> {
  let parsed = parse_part2(input)?;
  let mut aggregate: usize = 0;
  let mut aggregate_p: bool = true;
  for command in parsed {
    match command {
      Command::Do => aggregate_p = true,
      Command::Dont => aggregate_p = false,
      Command::Multiply(Mul(a,b)) => if aggregate_p {
        aggregate += a*b;
      }
    }
  }
  Ok(aggregate)
}