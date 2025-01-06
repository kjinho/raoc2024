use std::error::Error;
use nom::{
  branch::alt, 
  bytes::complete::tag, 
  character::complete::{anychar, char, u64}, 
  multi::many0, 
  sequence::{delimited, preceded, separated_pair}, 
  IResult
};

#[derive(Clone, Copy, Debug)]
struct Mul(usize,usize);

#[derive(Clone, Copy, Debug)]
enum Command {
  Multiply(Mul),
  Dont,
  Do,
  Nop
}

fn parse_dont(input: &str) -> IResult<&str, Command> {
  let (tail, _) = tag("don't()")(input)?;
  Ok((tail, Command::Dont))
}

fn parse_do(input: &str) -> IResult<&str, Command> {
  let (tail, _) = tag("do()")(input)?;
  Ok((tail, Command::Do))
}

fn parse_comma_num(input: &str) -> IResult<&str,(usize,usize)> {
  let (tail, (a, b)) = 
    separated_pair(u64,char(','),u64)(input)?;
  Ok((tail, (a as usize, b as usize)))
}

fn parse_mul(input: &str) -> IResult<&str, Command> {
  let (tail, (first, second)) = 
    preceded(tag("mul"),delimited(char('('),parse_comma_num,char(')')))(input)?;
  Ok((tail, Command::Multiply(Mul(first, second))))
}

fn parse_anything_else(input: &str) -> IResult<&str,Command> {
  let (tail, _) = anychar(input)?;
  Ok((tail, Command::Nop))
}

fn parse_ops_part2(input: &str) -> IResult<&str, Command> {
  alt((
    parse_dont,
    parse_do,
    parse_mul,
    parse_anything_else
  ))(input)
}

fn parse_input(input: &str) -> IResult<&str,Vec<Command>> {
  let (_, result) = many0(parse_ops_part2)(input)?;
  Ok(("", result))
}

fn exec_command_part1(command: Command, _: bool) -> (usize,bool) {
  match command {
    Command::Multiply(Mul(a,b)) => (a*b,true),
    _ => (0,true)
  }
}

fn exec_command_part2(command: Command, state: bool) -> (usize,bool) {
  match command {
    Command::Do => (0,true),
    Command::Dont => (0,false),
    Command::Multiply(Mul(a,b)) => if state {
      (a*b,true)
    } else {
      (0,false)
    },
    _ => (0,state)
  }
}

fn part_helper(input: &str, command_fn: fn(Command,bool)->(usize,bool)) -> Result<usize,Box<dyn Error>> {
  let (_, parsed) = 
    parse_input(input).map_err(|e| format!("Parsing error: {:?}", e))?;
  let mut aggregate: usize = 0;
  let mut aggregate_p: bool = true;
  for command in parsed {
    let (new_num, new_aggregate_p) = command_fn(command, aggregate_p);
    aggregate += new_num;
    aggregate_p = new_aggregate_p;
  }
  Ok(aggregate)
}

pub fn part1(input: &str) -> Result<usize,Box<dyn Error>> {
  part_helper(input, exec_command_part1)
}

pub fn part2(input: &str) -> Result<usize,Box<dyn Error>> {
  part_helper(input, exec_command_part2)
}