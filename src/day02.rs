use std::{error::Error, num::ParseIntError};


struct Report(Vec<isize>);

fn parse_report(s: &str) -> Result<Report,ParseIntError> {
    s.split_whitespace()
        .map(|x|x.parse::<isize>())
        .collect::<Result<Vec<isize>,_>>()
        .map(Report)
}

fn parse_input(input: &str) -> Result<Vec<Report>, ParseIntError> {
  input.lines().map(parse_report).collect::<Result<Vec<Report>,ParseIntError>>()
}

fn all_increasing_p(Report(level): &Report) -> bool {
  level.iter().is_sorted_by(| &a, &b| *a < *b)
}

fn all_decreasing_p(Report(level): &Report) -> bool {
  level.iter().is_sorted_by(| &a, &b| *a > *b)
}

fn permitted_diff_p(a: isize, b: isize) -> bool {
  let diff = a.abs_diff(b);
  diff >= 1 && diff <= 3
}

fn permitted_diff2_p(nums: &[isize]) -> bool {
  match nums {
    [first, second] => permitted_diff_p(*first, *second),
    _ => false
  }
}

fn all_permitted_diff(Report(levels): &Report) -> bool {
  levels.windows(2).all(permitted_diff2_p)
}

fn level_safe_p(report: &Report) -> bool {
  (all_increasing_p(report) || all_decreasing_p(report)) && all_permitted_diff(report)
}

pub fn part1(input: &str) -> Result<usize,Box<dyn Error>> {
  Ok(parse_input(input)?.iter().filter(|&x| level_safe_p(x)).count())
}

fn dampened_safe_p(Report(levels): &Report) -> bool {
  for (i,_) in levels.iter().enumerate() {
    let mut vec = levels.clone();
    vec.remove(i);
    if level_safe_p(&Report(vec)) {
      return true
    }
  }
  false
}

pub fn part2(input: &str) -> Result<usize,Box<dyn Error>> {
  Ok(parse_input(input)?.iter().filter(|&x| dampened_safe_p(x)).count())
}