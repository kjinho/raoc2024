use std::error::Error;
use std::fs;
use std::io;
//use std::str::Split;

fn load_input(day: usize, part: usize) -> io::Result<String> {
    let path = format!("data/day{:02}-{}.txt", day, part);
    fs::read_to_string(path)
}

fn main() -> Result<(), Box<dyn Error>> {
    for (day, part,day_func) in [
        (1usize,1usize,day01_part1 as fn(String) -> Result<usize,Box<dyn Error>>),
        (1,2,day01_part2 as fn(String) -> Result<usize,Box<dyn Error>>)
    ] {
        println!("Result for Day {}, Part {}", day, part);
        println!("| Output: {}", day_func(load_input(day, part)?)?);
    } 
    Ok(())
}

// day 01 

fn words(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

struct Pair(usize, usize);

fn parse_line(s: &str) -> Result<Pair,Box<dyn Error>> {
    let iter = words(s);
    let first = match iter.get(0) {
        Some(x) => Ok(*x),
        None => Err("No first word.")
    }?;
    let second = match iter.get(1) {
        Some(x) => Ok(*x),
        None => Err("No second word.")
    }?;
    let first_num = first.parse::<usize>()?;
    let second_num = second.parse::<usize>()?;
    Ok(Pair(first_num,second_num))
}

fn parse_input(input: &str) -> Result<Vec<Pair>, Box<dyn Error>> {
    let ls = input.lines();
    let mut res: Vec<Pair> = Vec::new();
    for line in ls {
        res.push(parse_line(line)?);
    }
    Ok(res)
}

fn unzip_pair_vec(ps: Vec<Pair>) -> (Vec<usize>,Vec<usize>) {
    let mut a: Vec<usize> = Vec::new();
    let mut b: Vec<usize> = Vec::new();
    for Pair(x,y) in ps {
        a.push(x);
        b.push(y);
    }
    (a,b)
}

fn sort_pair_vec(ps: Vec<Pair>) -> Vec<Pair> {
    let (mut a,mut b) = unzip_pair_vec(ps);
    a.sort();
    b.sort();
    let mut res: Vec<Pair> = Vec::new();
    for (x,y) in std::iter::zip(a,b) {
        res.push(Pair(x,y));
    }
    res
}

fn aggregate_differences(ps: Vec<Pair>) -> usize {
    let mut res: usize = 0;
    for Pair(x,y) in ps {
        res += x.abs_diff(y);
    }
    res
}

fn day01_part1(input: String) -> Result<usize,Box<dyn Error>> {
    let parsed = parse_input(input.as_str())?;
    Ok(aggregate_differences(sort_pair_vec(parsed)))
}

fn day01_part2(input: String) -> Result<usize,Box<dyn Error>> {
    let parsed = parse_input(input.as_str())?;
    let (a,b) = unzip_pair_vec(parsed);
    let mut res: usize = 0;
    for x in a {
        for y in &b {
            if x == *y {
                res += x;
            }
        }
    }
    Ok(res)
}