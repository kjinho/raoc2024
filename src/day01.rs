use std::error::Error;

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
    input.lines()
        .map(parse_line)
        .into_iter()
        .collect()
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
    a.iter().zip(b).map(|(&x,y)| Pair(x,y)).collect()
}

fn aggregate_differences(ps: Vec<Pair>) -> usize {
    ps.iter()
        .map(|&Pair(x,y)| x.abs_diff(y))
        .sum()
}

pub fn part1(input: &str) -> Result<usize,Box<dyn Error>> {
    let parsed = parse_input(input)?;
    Ok(aggregate_differences(sort_pair_vec(parsed)))
}

pub fn part2(input: &str) -> Result<usize,Box<dyn Error>> {
    let parsed = parse_input(input)?;
    let (a,b) = unzip_pair_vec(parsed);
    let res =
        a.iter()
            .map(|&x| 
                b.iter()
                    .filter(|&&y| x == y)
                    .sum::<usize>())
            .sum::<usize>();
    Ok(res)
}