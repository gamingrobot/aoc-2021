use std::{str::FromStr, error::Error};

#[aoc_generator(day2)]
fn gen(input: &str) -> Result<Vec<Command>, Box<dyn Error>> {
    let mut output = Vec::new();
    for line in input.lines() {
        let mut split = line.split(' ');
        let direction = split.next().unwrap().parse().unwrap();
        let unit = split.next().unwrap().parse().unwrap();
        output.push(Command::new(direction, unit))
    }
    Ok(output)
}

#[aoc(day2, part1)]
pub fn part1(input: &[Command]) -> u32 {
    let mut depth: u32 = 0;
    let mut horizontal: u32 = 0;
    for command in input {
        match command.direction {
            Direction::Forward => horizontal += command.unit,
            Direction::Down => depth += command.unit,
            Direction::Up => depth -= command.unit,
        }
    }
    println!("{:?}", depth);
    println!("{:?}", horizontal);
    horizontal * depth
}

#[aoc(day2, part2)]
pub fn part2(input: &[Command]) -> u32 {
    let mut depth: u32 = 0;
    let mut horizontal: u32 = 0;
    let mut aim: u32 = 0;
    for command in input {
        match command.direction {
            Direction::Forward => {
                horizontal += command.unit;
                depth += aim * command.unit;
            },
            Direction::Down => aim += command.unit,
            Direction::Up => aim -= command.unit,
        }
    }
    println!("{:?}", depth);
    println!("{:?}", horizontal);
    println!("{:?}", aim);
    horizontal * depth
}

#[derive(Debug)]
pub struct Command {
    direction: Direction,
    unit: u32,
}

impl Command {
    fn new(direction: Direction, unit: u32) -> Command {
        Command { direction, unit }
    }
}

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Direction, ()> {
        match s {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}
