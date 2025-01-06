use std::error::Error;

use nom::{
  bytes::complete::tag, 
  character::complete::{newline, u64},
  multi::separated_list1,
  sequence::{terminated, tuple}, 
  IResult};

struct Equation {
  test_value: usize, 
  numbers: Vec<usize>
}

fn parse_test_value(input: &str) -> IResult<&str, usize> {
  let (tail, num) = terminated(u64, tag(": "))(input)?;
  Ok((tail, num as usize))
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<usize>> {
  let (tail, res) = separated_list1(tag(" "), u64)(input)?;
  Ok((tail, res.iter().map(|&x| x as usize).collect()))
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
  let (tail, (test_value, numbers)) = 
    tuple((parse_test_value, parse_numbers))(input)?;
  Ok((tail, Equation {
    test_value,
    numbers
  }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Equation>> {
  separated_list1(newline, parse_equation)(input)
}

pub fn part1(input: &str) -> Result<usize,Box<dyn Error>> {
  let (_, parsed) = 
    parse_input(input).map_err(|e| format!("error {e}"))?;

  let result = 
    parsed.iter()
      .filter(|&eq| eq.makeable_true(&vec![Operator::Add, Operator::Multiply]))
      .map(|eq| eq.test_value).sum();

  Ok(result)
}

pub fn part2(input: &str) -> Result<usize,Box<dyn Error>> {
  let (_, parsed) = 
    parse_input(input).map_err(|e| format!("error {e}"))?;

  let result = 
    parsed.iter()
      .filter(|&eq| eq.makeable_true(&vec![Operator::Add, Operator::Multiply, Operator::Concat]))
      .map(|eq| eq.test_value).sum();

  Ok(result)
}

#[derive(Clone, Copy, Debug)]
enum Operator {
  Add,
  Multiply,
  Concat
}

impl Equation {
  fn operators(&self, ops: &Vec<Operator>) -> Vec<Vec<Operator>> {
    permute_operators(self.numbers.len() - 1, &ops)
  }

  fn exec_ops(&self, ops: &Vec<Operator>) -> Option<usize> {
    match self.numbers.as_slice() {
      [] => None,
      [x] => Some(*x),
      [first, rest @ ..] => {
        ops.iter().zip(rest).fold(
          Some(*first), 
          |acc, (&op, num)| {
            let acc = acc?;
            match op {
              Operator::Add => Some(acc+num),
              Operator::Multiply => Some(acc*num),
              Operator::Concat => {
                let number_string = format!("{acc}{num}");
                number_string.parse::<usize>().ok()
              }
            }
          }
        )
      }
    }
  }

  fn valid_p(&self, ops: &Vec<Operator>) -> bool {
    match self.exec_ops(ops) {
      None => false,
      Some(x) => x == self.test_value
    }
  }

  fn makeable_true(&self, ops: &Vec<Operator>) -> bool {
    let possibilities =
      self.operators(ops);
    possibilities.iter().any(|ops| self.valid_p(ops))
  }

}

fn permute_operators(n: usize, operators: &Vec<Operator>) -> Vec<Vec<Operator>> {
  match n {
    0 | 1 => operators.iter().map(|&op| vec![op]).collect(),
    n => {
      let prev = permute_operators(n - 1, operators);
      let mut result: Vec<Vec<Operator>> = Vec::new();
      for &op in operators {
        for ops in &prev {
          let mut a = ops.clone();
          a.push(op);
          result.push(a);
        }
      }
      result
    }
  }
}
